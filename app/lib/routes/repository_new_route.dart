import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/bridge/api/settings.dart';
import 'package:stride/routes/tasks_route.dart';
import 'package:stride/widgets/custom_app_bar.dart';
import 'package:uuid/uuid.dart';

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

  bool get _isFirstStep => _currentStep == 0;
  bool get _isLastStep => _currentStep + 1 == _steps().length;

  final _nameController = TextEditingController(text: 'my-repository');
  final _originController = TextEditingController(text: '');
  final _authorController = TextEditingController(text: defaultAuthorName);
  final _emailController = TextEditingController(text: defaultAuthorEmail);
  final _branchController = TextEditingController(text: defaultBranchName);
  // final _encrytionKeyController = TextEditingController();

  final GlobalKey<FormState> _generalFormKey = GlobalKey();
  final GlobalKey<FormState> _gitIntegrationFormKey = GlobalKey();
  // final GlobalKey<FormState> _encryptionFormKey = GlobalKey();

  List<Step> _steps() => [
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
          title: const Text('Git Integration'),
          content: _GitIntegrationRepositoryForm(
            formKey: _gitIntegrationFormKey,
            author: _authorController,
            email: _emailController,
            branch: _branchController,
            origin: _originController,
          ),
        ),
        // Step(
        //   state: _currentStep > 2 ? StepState.complete : StepState.indexed,
        //   title: const Text('Encryption'),
        //   content: _EncryptionRepositoryForm(
        //     formKey: _encryptionFormKey,
        //     encryptionKey: _encrytionKeyController,
        //   ),
        // ),
      ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: const CustomAppBar(title: 'New Repository'),
      body: Stepper(
        steps: _steps(),
        currentStep: _currentStep,
        onStepContinue: _onStepContinue,
        onStepCancel: _onStepCancel,
        onStepTapped: _onStepTapped,
        controlsBuilder: _controlsBuilder,
      ),
    );
  }

  Future<void> _onStepContinue() async {
    if (!_isLastStep) {
      setState(() => _currentStep += 1);
      return;
    }
    final repositoryUuid = const Uuid().v7obj();
    final settings = context.read<SettingsBloc>().settings;
    context.read<SettingsBloc>().add(
          SettingsUpdateEvent(
            settings: settings.copyWith(
              repositories: settings.repositories.toList()
                ..add(
                  Repository(
                    uuid: repositoryUuid,
                    name: _nameController.text,
                    origin: '',
                    author: _authorController.text,
                    email: _emailController.text,
                    branch: _branchController.text,
                  ),
                ),
              currentRepository: repositoryUuid,
            ),
          ),
        );
    Navigator.of(context).pushReplacement(
      MaterialPageRoute<void>(
        builder: (context) => const TasksRoute(),
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
            onPressed: details.onStepContinue,
            style: ElevatedButton.styleFrom(
              elevation: 4,
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(5), // Set the radius here
              ),
            ),
            child: Text(_isLastStep ? 'Confirm' : 'Next'),
          ),
          const SizedBox(width: 16),
          if (!_isFirstStep)
            ElevatedButton(
              onPressed: details.onStepCancel,
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

  const _GeneralRepositoryForm({
    required this.formKey,
    required this.name,
  });

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
          ),
        ],
      ),
    );
  }
}

class _GitIntegrationRepositoryForm extends StatelessWidget {
  const _GitIntegrationRepositoryForm({
    required this.formKey,
    required this.author,
    required this.email,
    required this.branch,
    required this.origin,
  });

  final GlobalKey<FormState> formKey;
  final TextEditingController author;
  final TextEditingController email;
  final TextEditingController branch;
  final TextEditingController origin;

  @override
  Widget build(BuildContext context) {
    return Form(
      key: formKey,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          TextFormField(
            controller: author,
            decoration: const InputDecoration(
              hintText: 'Enter Commit Author',
            ),
            validator: (value) {
              if (value == null || value.isEmpty) {
                return 'Please enter some text';
              }
              return null;
            },
          ),
          TextFormField(
            controller: email,
            decoration: const InputDecoration(
              hintText: 'Enter Commit Email',
            ),
            validator: (value) {
              if (value == null || value.isEmpty) {
                return 'Please enter some text';
              }
              return null;
            },
          ),
          TextFormField(
            controller: branch,
            decoration: const InputDecoration(hintText: 'Enter Branch Name'),
            validator: (value) {
              if (value == null || value.isEmpty) {
                return 'Please enter some text';
              }
              return null;
            },
          ),
          TextFormField(
            controller: origin,
            decoration: const InputDecoration(hintText: defaultOriginHint),
            validator: (value) {
              // TODO: Add git URL validation.
              return null;
            },
          ),
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
              hintText: 'Enter Encrypiton Key',
            ),
            validator: (value) {
              if (value == null || value.isEmpty) {
                return 'Please enter some text';
              }
              return null;
            },
          ),
        ],
      ),
    );
  }
}
