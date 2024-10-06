import 'package:flutter/material.dart';

class InitialRoute extends StatelessWidget {
  const InitialRoute({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Welcome to stride')),
      body: Center(
        child: Column(
          children: [
            ElevatedButton.icon(
              onPressed: () {},
              label: Text('New Repository'),
            ),
            ElevatedButton.icon(
              onPressed: () {},
              label: Text('Clone Repository'),
            ),
          ],
        ),
      ),
    );
  }
}
