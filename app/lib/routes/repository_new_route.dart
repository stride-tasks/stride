import 'package:flutter/material.dart';
import 'package:stride/widgets/custom_app_bar.dart';

class RepositoryNewRoute extends StatefulWidget {
  const RepositoryNewRoute({super.key});

  @override
  State<RepositoryNewRoute> createState() => _RepositoryNewRouteState();
}

class _RepositoryNewRouteState extends State<RepositoryNewRoute> {
  int _currentStep = 0;

  bool get _isFirstStep => _currentStep == 0;
  bool get _isLastStep => _currentStep + 1 == _steps().length;

  final _nameController = TextEditingController();
  final _authorController = TextEditingController();
  final _emailController = TextEditingController();
  final _branchController = TextEditingController();
  final _encrytionKeyController = TextEditingController();

  final GlobalKey<FormState> _generalFormKey = GlobalKey();
  final GlobalKey<FormState> _gitIntegrationFormKey = GlobalKey();
  final GlobalKey<FormState> _encryptionFormKey = GlobalKey();

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
          ),
        ),
        Step(
          state: _currentStep > 2 ? StepState.complete : StepState.indexed,
          title: const Text('Encryption'),
          content: _EncryptionRepositoryForm(
            formKey: _encryptionFormKey,
            encryptionKey: _encrytionKeyController,
          ),
        ),
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

  void _onStepContinue() {
    if (_isLastStep) {
    } else {
      setState(() => _currentStep += 1);
    }
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
  });

  final GlobalKey<FormState> formKey;
  final TextEditingController author;
  final TextEditingController email;
  final TextEditingController branch;

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
              hintText: 'Enter Commit Author Name',
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
              hintText: 'Enter Commit Email Name',
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
