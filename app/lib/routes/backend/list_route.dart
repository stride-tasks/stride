import 'package:flutter/material.dart';
import 'package:stride/bridge/api/repository.dart';
import 'package:stride/routes/backend/config_route.dart';
import 'package:stride/widgets/settings_widget.dart';

class BackendListRoute extends StatefulWidget {
  final Repository repository;
  const BackendListRoute({super.key, required this.repository});

  @override
  State<BackendListRoute> createState() => _BackendListRouteState();
}

class _BackendListRouteState extends State<BackendListRoute> {
  TextStyle get headingStyle => const TextStyle(
    fontSize: 16,
    fontWeight: FontWeight.w600,
    color: Colors.red,
  );

  Future<List<BackendRecord>>? _backends;
  Future<List<String>>? _backendNames;

  @override
  void initState() {
    super.initState();

    _backends = widget.repository.backends();
    _backendNames = widget.repository.backendNames();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Backends')),
      body: FutureBuilder(
        future: _backends,
        builder: (context, snapshot) {
          if (snapshot.connectionState != ConnectionState.done) {
            // TODO: Factor this common pattern into a function.
            return Center(child: CircularProgressIndicator.adaptive());
          }
          final backends = snapshot.data!.map((backend) {
            return SettingsTileNavigation(
              title: Text(backend.name),
              leading: const Icon(Icons.task),
              trailing: Switch(
                value: backend.enabled,
                activeColor: Colors.redAccent,
                onChanged: (value) async {
                  await widget.repository.updateBackend(
                    backend: BackendRecord(
                      id: backend.id,
                      name: backend.name,
                      enabled: !backend.enabled,
                      config: backend.config,
                    ),
                  );
                  setState(() {
                    _backends = widget.repository.backends();
                  });
                },
              ),
              builder: (context) => BackendConfigRoute(
                repository: widget.repository,
                backendId: backend.id,
              ),
            );
          }).toList();
          return SettingsList(
            sections: [
              SettingsSection(
                title: Text('Backends', style: headingStyle),
                tiles: backends,
              ),
            ],
          );
        },
      ),
      floatingActionButton: FutureBuilder(
        future: _backendNames,
        builder: (context, snapshot) {
          if (snapshot.connectionState != ConnectionState.done) {
            return Center();
          }

          final names = snapshot.data!;
          return FloatingActionButton(
            shape: const CircleBorder(),
            onPressed: () async {
              await showModalBottomSheet<void>(
                context: context,
                builder: (context) {
                  return ListView.builder(
                    itemCount: names.length,
                    itemBuilder: (context, index) {
                      final name = names[index];
                      return ListTile(
                        title: Text(name),
                        onTap: () async {
                          await widget.repository.addBackend(name: name);
                          setState(() {
                            _backends = widget.repository.backends();
                            _backendNames = widget.repository.backendNames();
                          });
                        },
                      );
                    },
                  );
                },
              );
            },
            child: const Icon(Icons.add, size: 50),
          );
        },
      ),
    );
  }
}
