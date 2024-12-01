import 'package:flutter/material.dart';
import 'package:stride/routes/repository_new_route.dart';
import 'package:stride/widgets/custom_app_bar.dart';

class InitialRoute extends StatelessWidget {
  const InitialRoute({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: const CustomAppBar(title: ''),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            SizedBox(
              width: 250,
              height: 50,
              child: ElevatedButton.icon(
                onPressed: () {
                  Navigator.of(context).push<void>(
                    MaterialPageRoute(
                      builder: (context) => const RepositoryNewRoute(),
                    ),
                  );
                },
                label: const Text('New Repository'),
                icon: const Icon(Icons.add),
                style: ElevatedButton.styleFrom(
                  elevation: 4,
                  shape: RoundedRectangleBorder(
                    borderRadius:
                        BorderRadius.circular(5), // Set the radius here
                  ),
                ),
              ),
            ),
            const SizedBox(height: 12),
            SizedBox(
              width: 250,
              height: 50,
              child: ElevatedButton.icon(
                onPressed: () {},
                label: const Text('Clone Repository'),
                icon: const Icon(Icons.cloud),
                style: ElevatedButton.styleFrom(
                  elevation: 4,
                  shape: RoundedRectangleBorder(
                    borderRadius:
                        BorderRadius.circular(5), // Set the radius here
                  ),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
