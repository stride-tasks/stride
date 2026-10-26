import 'dart:convert';
import 'dart:math';
import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:stride/api.dart';
import 'package:stride/context.dart';
import 'package:stride/logging.dart';
import 'package:stride/routes/ssh_keys_route.dart';
import 'package:stride/routes/tasks_route.dart';
import 'package:stride/widgets/custom_app_bar.dart';
import 'package:stride/widgets/icon_text_button.dart';
import 'package:uuid/uuid_value.dart';

class RepositoryNewRoute extends StatefulWidget {
  final bool cloning;
  const RepositoryNewRoute({super.key, this.cloning = false});

  @override
  State<RepositoryNewRoute> createState() => _RepositoryNewRouteState();
}

const String defaultOriginHint = 'git@github.com:user/repo.git';
const String defaultAuthorName = 'stride';
const String defaultAuthorEmail = 'noreply.stride.tasks@gmail.com';
const String defaultBranchName = 'main';

class _RepositoryNewRouteState extends State<RepositoryNewRoute> {
  int _currentStep = 0;
  bool _isSubmitting = false;
  bool _generatedKeyShown = false;
  UuidValue? _createdRepositoryId;
  UuidValue? _gitBackendId;
  Uint8List? _generatedEncryptionKey;

  bool get _isFirstStep => _currentStep == 0;
  bool get _isLastStep => _currentStep + 1 == _steps().length;

  final _nameController = TextEditingController(text: 'my-repository');
  final _originController = TextEditingController();
  final _authorController = TextEditingController(text: defaultAuthorName);
  final _emailController = TextEditingController(text: defaultAuthorEmail);
  final _branchController = TextEditingController(text: defaultBranchName);
  final _sshKeyController = TextEditingController();
  final _encrytionKeyController = TextEditingController();

  final GlobalKey<FormState> _generalFormKey = GlobalKey();
  final GlobalKey<FormState> _gitIntegrationFormKey = GlobalKey();
  final GlobalKey<FormState> _encryptionFormKey = GlobalKey();

  List<Step> _steps() {
    final steps = [
      Step(
        state: _currentStep > 0 ? StepState.complete : StepState.indexed,
        title: const Text('General'),
        content: _GeneralRepositoryForm(
          formKey: _generalFormKey,
          name: _nameController,
        ),
      ),
      Step(
        state: _currentStep > 1 ? StepState.complete : StepState.indexed,
        title: Text(
          widget.cloning ? 'Git Integration' : 'Git Integration (optional)',
        ),
        content: _GitIntegrationRepositoryForm(
          formKey: _gitIntegrationFormKey,
          author: _authorController,
          email: _emailController,
          branch: _branchController,
          origin: _originController,
          sshKey: _sshKeyController,
          cloning: widget.cloning,
        ),
      ),
    ];
    if (widget.cloning) {
      steps.add(
        Step(
          state: _currentStep > 2 ? StepState.complete : StepState.indexed,
          title: const Text('Encryption'),
          content: _EncryptionRepositoryForm(
            formKey: _encryptionFormKey,
            encryptionKey: _encrytionKeyController,
          ),
        ),
      );
    }

    return steps;
  }

  List<GlobalKey<FormState>> _formKeys() => [
    _generalFormKey,
    _gitIntegrationFormKey,
    if (widget.cloning) _encryptionFormKey,
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: CustomAppBar(
        title: widget.cloning ? 'Clone Repository' : 'New Repository',
      ),
      body: Stepper(
        steps: _steps(),
        currentStep: _currentStep,
        onStepContinue: _isSubmitting ? null : _onStepContinue,
        onStepCancel: _isSubmitting ? null : _onStepCancel,
        onStepTapped: _isSubmitting ? null : _onStepTapped,
        controlsBuilder: _controlsBuilder,
      ),
    );
  }

  @override
  void dispose() {
    _nameController.dispose();
    _originController.dispose();
    _authorController.dispose();
    _emailController.dispose();
    _branchController.dispose();
    _sshKeyController.dispose();
    _encrytionKeyController.dispose();
    super.dispose();
  }

  Future<void> _onStepContinue() async {
    final form = _formKeys()[_currentStep].currentState;
    if (form == null || !form.validate()) {
      return;
    }

    if (!_isLastStep) {
      setState(() => _currentStep += 1);
      return;
    }

    setState(() => _isSubmitting = true);
    try {
      final repositoryId =
          _createdRepositoryId ??=
              (await RepositoryAddMethod(
                name: _nameController.text.trim(),
              ).execute()).repositoryId;
      final hasRemote = _originController.text.trim().isNotEmpty;

      if (widget.cloning || hasRemote) {
        final encryptionKey = widget.cloning
            ? _decodeEncryptionKey()
            : (_generatedEncryptionKey ??= _generateEncryptionKey());

        await _configureGitBackend(repositoryId, encryptionKey);

        if (!widget.cloning && !_generatedKeyShown) {
          await _showGeneratedEncryptionKey(encryptionKey);
          _generatedKeyShown = true;
        }

        await RepositorySyncMethod(id: repositoryId).execute();
      }

      await RustContext.refreshRepositories();
      if (!mounted) return;

      Navigator.of(context).pushReplacement(
        MaterialPageRoute<void>(builder: (context) => const TasksRoute()),
      );
    } on Exception catch (error) {
      await Log.error('Repository setup failed: $error');
      if (!mounted) return;

      final message = _createdRepositoryId == null
          ? 'Could not create the repository: $error'
          : 'Repository setup failed. Retry to resume: $error';
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text(message)));
    } finally {
      if (mounted) {
        setState(() => _isSubmitting = false);
      }
    }
  }

  Future<void> _configureGitBackend(
    UuidValue repositoryId,
    Uint8List encryptionKey,
  ) async {
    var backendId = _gitBackendId;
    if (backendId == null) {
      final existingBackends = await RepositoryBackendListMethod(
        repositoryId: repositoryId,
      ).execute();
      backendId = existingBackends.backends
          .where((backend) => backend.name == 'git')
          .firstOrNull
          ?.id;
      if (backendId == null) {
        backendId = (await RepositoryBackendAddMethod(
          repositoryId: repositoryId,
          backend: 'git',
        ).execute()).backendId;
      }
      _gitBackendId = backendId;
    }

    final result = await RepositoryBackendGetMethod(
      repositoryId: repositoryId,
      backendId: backendId,
    ).execute();
    final record = result.backendRecord;
    if (record == null) {
      throw Exception('Unable to load the Git backend configuration.');
    }

    final config = Config(
      fields: [
        ConfigField(
          id: 'origin',
          value: ConfigValue.string(value: _originController.text.trim()),
        ),
        ConfigField(
          id: 'author',
          value: ConfigValue.string(value: _authorController.text.trim()),
        ),
        ConfigField(
          id: 'email',
          value: ConfigValue.string(value: _emailController.text.trim()),
        ),
        ConfigField(
          id: 'branch',
          value: ConfigValue.string(value: _branchController.text.trim()),
        ),
        ConfigField(
          id: 'encryption_key',
          value: ConfigValue.bytes(value: encryptionKey),
        ),
        ConfigField(
          id: 'ssh_key',
          value: ConfigValue.uuid(
            value: UuidValue.fromString(_sshKeyController.text),
          ),
        ),
      ],
    );

    await RepositoryBackendSetMethod(
      repositoryId: repositoryId,
      backendInstance: record.instance.copyWith(config: config),
    ).execute();

    if (record.instance.state != BackendInstanceState.enabled) {
      await RepositoryBackendToggleMethod(
        repositoryId: repositoryId,
        backendId: backendId,
      ).execute();
    }
  }

  Uint8List _decodeEncryptionKey() {
    final key = _tryDecodeEncryptionKey(_encrytionKeyController.text.trim());
    if (key == null || key.length != 32) {
      throw const FormatException(
        'Encryption key must be valid URL-safe Base64 for 32 bytes.',
      );
    }
    return key;
  }

  Uint8List _generateEncryptionKey() {
    final random = Random.secure();
    return Uint8List.fromList(
      List<int>.generate(32, (_) => random.nextInt(256)),
    );
  }

  Future<void> _showGeneratedEncryptionKey(Uint8List key) async {
    await showDialog<void>(
      context: context,
      barrierDismissible: false,
      builder: (context) => AlertDialog(
        title: const Text('Save your encryption key'),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              'Save this key somewhere safe. You will need it to access this '
              'repository on other devices.',
            ),
            const SizedBox(height: 12),
            SelectableText(base64Url.encode(key)),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Continue'),
          ),
        ],
      ),
    );
  }

  void _onStepTapped(int value) => setState(() => _currentStep = value);
  void _onStepCancel() =>
      _isFirstStep ? null : setState(() => _currentStep -= 1);

  Widget _controlsBuilder(BuildContext context, ControlsDetails details) {
    return Padding(
      padding: const EdgeInsets.only(top: 32),
      child: Row(
        children: [
          ElevatedButton(
            onPressed: _isSubmitting ? null : details.onStepContinue,
            style: ElevatedButton.styleFrom(
              elevation: 4,
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(5), // Set the radius here
              ),
            ),
            child: _isSubmitting
                ? const SizedBox(
                    width: 20,
                    height: 20,
                    child: CircularProgressIndicator.adaptive(
                      strokeWidth: 2,
                    ),
                  )
                : Text(
                    _isLastStep
                        ? widget.cloning
                              ? 'Clone Repository'
                              : 'Create Repository'
                        : 'Next',
                  ),
          ),
          const SizedBox(width: 16),
          if (!_isFirstStep)
            ElevatedButton(
              onPressed: _isSubmitting ? null : details.onStepCancel,
              style: ElevatedButton.styleFrom(
                elevation: 4,
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(5), // Set the radius here
                ),
              ),
              child: const Text('Back'),
            ),
        ],
      ),
    );
  }
}

class _GeneralRepositoryForm extends StatelessWidget {
  final TextEditingController name;
  final GlobalKey<FormState> formKey;

  const _GeneralRepositoryForm({required this.formKey, required this.name});

  void _onChange(String _) {
    formKey.currentState!.validate();
  }

  @override
  Widget build(BuildContext context) {
    return Form(
      key: formKey,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          TextFormField(
            controller: name,
            decoration: const InputDecoration(
              hintText: 'Enter Repository Name',
            ),
            validator: (value) {
              if (value == null || value.isEmpty) {
                return 'Please enter some text';
              }
              return null;
            },
            onChanged: _onChange,
          ),
        ],
      ),
    );
  }
}

class _GitIntegrationRepositoryForm extends StatefulWidget {
  const _GitIntegrationRepositoryForm({
    required this.formKey,
    required this.author,
    required this.email,
    required this.branch,
    required this.origin,
    required this.sshKey,
    this.cloning = false,
  });

  final GlobalKey<FormState> formKey;
  final TextEditingController author;
  final TextEditingController email;
  final TextEditingController branch;
  final TextEditingController origin;
  final TextEditingController sshKey;
  final bool cloning;

  @override
  State<_GitIntegrationRepositoryForm> createState() =>
      _GitIntegrationRepositoryFormState();
}

class _GitIntegrationRepositoryFormState
    extends State<_GitIntegrationRepositoryForm> {
  void _onChange(String _) {
    widget.formKey.currentState?.validate();
    setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    return Form(
      key: widget.formKey,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          if (!widget.cloning)
            const Text(
              'Leave the remote empty to create a local-only repository.',
            ),
          TextFormField(
            controller: widget.origin,
            decoration: const InputDecoration(
              labelText: 'Remote origin',
              hintText: defaultOriginHint,
            ),
            validator: (value) {
              if (value == null || value.trim().isEmpty) {
                if (!widget.cloning) {
                  return null;
                }
                return 'Enter a remote Git origin.';
              }
              return null;
            },
            onChanged: _onChange,
            autovalidateMode: AutovalidateMode.always,
          ),
          if (widget.cloning || widget.origin.text.trim().isNotEmpty) ...[
            TextFormField(
              controller: widget.author,
              decoration: const InputDecoration(
                labelText: 'Commit author',
                hintText: 'Enter Commit Author',
              ),
              validator: (value) {
                if (value == null || value.trim().isEmpty) {
                  return 'Enter a commit author.';
                }
                return null;
              },
              onChanged: _onChange,
            ),
            TextFormField(
              controller: widget.email,
              decoration: const InputDecoration(
                labelText: 'Commit email',
                hintText: 'Enter Commit Email',
              ),
              validator: (value) {
                if (value == null || value.trim().isEmpty) {
                  return 'Enter a commit email.';
                }
                return null;
              },
              onChanged: _onChange,
            ),
            TextFormField(
              controller: widget.branch,
              decoration: const InputDecoration(
                labelText: 'Git branch',
                hintText: 'Enter Branch Name',
              ),
              validator: (value) {
                if (value == null || value.trim().isEmpty) {
                  return 'Enter a branch name.';
                }
                return null;
              },
              onChanged: _onChange,
            ),
            FormField(
              builder: (field) {
                final errorText = field.errorText;
                return Padding(
                  padding: const EdgeInsets.symmetric(vertical: 8),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Row(
                        children: [
                          IconTextButton(
                            icon: const Icon(Icons.key),
                            text: 'Choose SSH Key',
                            onPressed: () async {
                              await Navigator.of(context).push(
                                MaterialPageRoute<SshKey>(
                                  builder: (context) => SshKeysRoute(
                                    onTap: (key) {
                                      widget.sshKey.text = key.id.toString();
                                      Navigator.of(context).pop();
                                    },
                                    hasDelete: false,
                                  ),
                                ),
                              );
                              setState(() {});
                            },
                          ),
                          const SizedBox(width: 8),
                          if (widget.sshKey.text.isNotEmpty)
                            const Icon(Icons.check),
                        ],
                      ),
                      if (errorText != null)
                        Padding(
                          padding: const EdgeInsets.only(left: 12, top: 4),
                          child: Text(
                            errorText,
                            style: TextStyle(
                              color: Theme.of(context).colorScheme.error,
                            ),
                          ),
                        ),
                    ],
                  ),
                );
              },
              validator: (value) {
                if (widget.sshKey.text.isEmpty &&
                    (widget.cloning ||
                        widget.origin.text.trim().isNotEmpty)) {
                  return 'Choose an SSH key.';
                }
                return null;
              },
            ),
          ],
        ],
      ),
    );
  }
}

class _EncryptionRepositoryForm extends StatelessWidget {
  const _EncryptionRepositoryForm({
    required this.formKey,
    required this.encryptionKey,
  });

  final GlobalKey<FormState> formKey;
  final TextEditingController encryptionKey;

  void _onChange(String _) {
    formKey.currentState!.validate();
  }

  @override
  Widget build(BuildContext context) {
    return Form(
      key: formKey,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          TextFormField(
            controller: encryptionKey,
            decoration: const InputDecoration(
              labelText: 'Encryption key',
              hintText: 'Enter a URL-safe Base64 key',
            ),
            validator: _encryptionKeyError,
            onChanged: _onChange,
          ),
        ],
      ),
    );
  }
}

String? _encryptionKeyError(String? value) {
  if (value == null || value.trim().isEmpty) {
    return 'Enter an encryption key.';
  }

  final key = _tryDecodeEncryptionKey(value.trim());
  if (key == null) {
    return 'Enter a valid URL-safe Base64 key.';
  }
  if (key.length != 32) {
    return 'Encryption key must be 32 bytes.';
  }
  return null;
}

Uint8List? _tryDecodeEncryptionKey(String value) {
  try {
    return Uint8List.fromList(base64Url.decode(value));
  } on FormatException {
    return null;
  }
}
