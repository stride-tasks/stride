import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:isolate';
import 'dart:ui';

import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';
import 'package:stride/bridge/api/background.dart' as background;
import 'package:stride/bridge/api/error.dart';
import 'package:stride/bridge/api/logging.dart' as logging;
import 'package:stride/bridge/frb_generated.dart';
import 'package:uuid/uuid.dart';
import 'package:workmanager/workmanager.dart';

@immutable
abstract class BackgroundTask {
  String uniqueName();
  String taskName() {
    return uniqueName();
  }

  Map<String, dynamic> toInputData();
}

@immutable
class TaskSyncBackgroundTask implements BackgroundTask {
  final UuidValue repositoryId;
  const TaskSyncBackgroundTask({required this.repositoryId});

  @override
  Map<String, dynamic> toInputData() {
    return {
      'repository': {'id': repositoryId.toString()},
    };
  }

  @override
  String uniqueName() {
    return '${taskName()}:$repositoryId';
  }

  @override
  String taskName() {
    return 'task.sync';
  }
}

@immutable
class Output {
  final Name name;
  final Map<String, dynamic> inputData;
  final Object? error;
  final bool done;
  const Output({
    required this.name,
    required this.inputData,
    this.error,
    this.done = false,
  });

  @override
  bool operator ==(Object other) {
    if (other is! Output) {
      return false;
    }
    return name == other.name;
  }

  @override
  int get hashCode => name.hashCode;

  @override
  String toString() => '$name :: done:$done, error:$error';
}

@immutable
class Background {
  static late Set<Output>? _outputs;
  static late StreamController<Set<Output>>? _streamController;
  static late Stream<Set<Output>>? _stream;
  static late ReceivePort? _receivePort;

  static Stream<Set<Output>> stream() => _stream!;

  static Future<void> init() async {
    _outputs = {};
    _streamController = StreamController();

    _receivePort = ReceivePort('worker');
    _receivePort?.listen((message) {
      final output = message as Output;
      print('Background :: $output');

      _outputs?.remove(output);
      _outputs!.add(output);
      _streamController!.sink.add({...?_outputs});
    });
    _stream = _streamController?.stream.asBroadcastStream();

    IsolateNameServer.removePortNameMapping(_GLOBAL_PORT_NAME);
    IsolateNameServer.registerPortWithName(
      _receivePort!.sendPort,
      _GLOBAL_PORT_NAME,
    );

    if (Platform.isLinux || Platform.isMacOS || Platform.isWindows) {
      WorkmanagerPlatform.instance = await _DesktopWorkmanager.create();
    }
    await Workmanager().initialize(_callbackDispatcher);
  }

  static Future<void> run(
    BackgroundTask task, {
    Duration? initialDelay,
    ExistingWorkPolicy? existingWorkPolicy,
  }) async {
    final uniqueName = task.uniqueName();
    final taskName = task.taskName();
    return Workmanager().registerOneOffTask(
      uniqueName,
      taskName,
      inputData: task.toInputData(),
      initialDelay: initialDelay,
      existingWorkPolicy: existingWorkPolicy,
    );
  }

  static Future<void> periodic(
    BackgroundTask task, {
    required Duration frequency,
    Duration? initialDelay,
    ExistingPeriodicWorkPolicy? existingWorkPolicy,
  }) async {
    final uniqueName = task.uniqueName();
    final taskName = task.taskName();
    return Workmanager().registerPeriodicTask(
      uniqueName,
      taskName,
      inputData: task.toInputData(),
      initialDelay: initialDelay,
      frequency: frequency,
      existingWorkPolicy: existingWorkPolicy,
    );
  }
  static Future<void> cancel(BackgroundTask task) async {
    return Workmanager().cancelByUniqueName(task.uniqueName());
  }

  static Future<void> cancelAll() async {
    return Workmanager().cancelAll();
  }
}

@pragma('vm:entry-point')
void _callbackDispatcher() {
  Workmanager().executeTask(_executeTask);
}

const String _GLOBAL_PORT_NAME = 'worker-port';

Future<bool> _executeTask(String task, Map<String, dynamic>? inputData) async {
  final input = inputData ?? {};

  await RustLib.init();

  logging.trace(message: 'Background task: $task, inputData: $inputData');

  final port = IsolateNameServer.lookupPortByName(_GLOBAL_PORT_NAME);

  final name = Name.fromString(task);

  port?.send(Output(name: name, inputData: input));
  try {
    await background.execute(
      task: jsonEncode({'method': name.method, 'params': input}),
    );
    port?.send(Output(name: name, inputData: input, done: true));
  } on RustError catch (e) {
    port?.send(
      Output(
        name: name,
        inputData: input,
        error: e.toErrorString(),
        done: true,
      ),
    );
  }
  // ignore: avoid_catches_without_on_clauses
  catch (error) {
    port?.send(Output(name: name, inputData: input, error: error, done: true));
  }

  return Future.value(true);
}

@immutable
class Name {
  final String method;
  final String? unique;

  const Name({required this.method, this.unique});

  factory Name.fromString(String value) {
    final separatorIndex = value.indexOf(':');
    if (separatorIndex == -1) {
      return Name(method: value);
    }

    return Name(
      method: value.substring(0, separatorIndex),
      unique: value.substring(separatorIndex + 1),
    );
  }

  @override
  bool operator ==(Object other) {
    if (other is! Name) {
      return false;
    }
    return method == other.method && unique == other.unique;
  }

  @override
  int get hashCode => Object.hash(method, unique);

  @override
  String toString() {
    if (unique == null) {
      return method;
    } else {
      return '$method:$unique';
    }
  }
}

@immutable
class ComputeArgs {
  final RootIsolateToken token;
  final BTask task;
  const ComputeArgs({required this.token, required this.task});
}

@pragma('vm:entry-point')
Future<void> _compute(ComputeArgs args) async {
  BackgroundIsolateBinaryMessenger.ensureInitialized(args.token);
  await _executeTask(args.task.name.toString(), args.task.inputData);
}

@pragma('vm:entry-point')
Future<void> _entryPoint(SendPort sender) async {
  final receiver = ReceivePort('worker');
  sender.send(receiver.sendPort);

  final stream = receiver.asBroadcastStream();
  final token = (await stream.first) as RootIsolateToken;

  await RustLib.init();

  final tasks = <Name, Timer>{};
  final tagIndex = <String, Set<Name>>{};

  await for (final value in stream) {
    if (value is _CancelByName) {
      logging.info(message: 'Canceling task by name: ${value.uniqueName}');
      final name = Name.fromString(value.uniqueName);
      tasks.remove(name)?.cancel();
      continue;
    }
    if (value is _CancelByTag) {
      logging.info(message: 'Canceling tasks by tag: ${value.tag}');
      final names = tagIndex.remove(value.tag) ?? {};
      for (final name in names) {
        tasks.remove(name)?.cancel();
      }
      continue;
    }
    if (value is _CancelAll) {
      for (final timer in tasks.values) {
        timer.cancel();
      }
      tasks.clear();
      tagIndex.clear();
      continue;
    }

    final task = value as BTask;
    logging.trace(message: '${task.name}');

    if (task.frequency == null) {
      compute(_compute, ComputeArgs(token: token, task: task));
    } else {
      tasks[task.name]?.cancel();
      tasks[task.name] = Timer.periodic(task.frequency!, (timer) async {
        await compute(_compute, ComputeArgs(token: token, task: task));
      });
      if (task.tag != null) {
        tagIndex.putIfAbsent(task.tag!, () => {}).add(task.name);
      }
    }
  }
  logging.info(message: 'DONE');
}

@immutable
class BTask {
  final Name name;
  final Duration? frequency;
  final Map<String, dynamic> inputData;
  final String? tag;

  const BTask({required this.name, required this.inputData, this.frequency, this.tag});
}

@immutable
class _CancelByName {
  final String uniqueName;
  const _CancelByName(this.uniqueName);
}

@immutable
class _CancelByTag {
  final String tag;
  const _CancelByTag(this.tag);
}

@immutable
class _CancelAll {
  const _CancelAll();
}

@immutable
class _DesktopWorkmanager extends WorkmanagerPlatform {
  final Isolate _isolate;
  final ReceivePort _receiver;
  final SendPort _sender;

  _DesktopWorkmanager._internal({
    required Isolate isolate,
    required ReceivePort receiver,
    required SendPort sender,
  }) : _sender = sender,
       _receiver = receiver,
       _isolate = isolate;

  static Future<_DesktopWorkmanager> create() async {
    final receiver = ReceivePort('main');
    final isolate = await Isolate.spawn(
      _entryPoint,
      receiver.sendPort,
      debugName: 'background-worker',
    );

    final stream = receiver.asBroadcastStream();
    final sender = (await stream.first) as SendPort;

    // ignore: cascade_invocations
    sender.send(RootIsolateToken.instance!);

    return _DesktopWorkmanager._internal(
      isolate: isolate,
      receiver: receiver,
      sender: sender,
    );
  }

  @override
  Future<void> initialize(
    Function callbackDispatcher, {
    @Deprecated(
      'Use WorkmanagerDebug handlers instead. This parameter has no effect.',
    )
    bool isInDebugMode = false,
  }) async {}

  @override
  Future<void> registerOneOffTask(
    String uniqueName,
    String taskName, {
    Map<String, dynamic>? inputData,
    Duration? initialDelay,
    Constraints? constraints,
    ExistingWorkPolicy? existingWorkPolicy,
    BackoffPolicy? backoffPolicy,
    Duration? backoffPolicyDelay,
    String? tag,
    OutOfQuotaPolicy? outOfQuotaPolicy,
  }) async {
    _sender.send(
      BTask(name: Name.fromString(uniqueName), inputData: inputData ?? {}, tag: tag),
    );
  }

  @override
  Future<void> registerPeriodicTask(
    String uniqueName,
    String taskName, {
    Duration? frequency,
    Duration? flexInterval,
    Map<String, dynamic>? inputData,
    Duration? initialDelay,
    Constraints? constraints,
    ExistingPeriodicWorkPolicy? existingWorkPolicy,
    BackoffPolicy? backoffPolicy,
    Duration? backoffPolicyDelay,
    String? tag,
  }) async {
    _sender.send(
      BTask(
        name: Name.fromString(uniqueName),
        inputData: inputData ?? {},
        frequency: frequency,
        tag: tag,
      ),
    );
  }

  @override
  Future<void> registerProcessingTask(
    String uniqueName,
    String taskName, {
    Duration? initialDelay,
    Map<String, dynamic>? inputData,
    Constraints? constraints,
  }) async {
    throw UnimplementedError(
      'No implementation found for workmanager on this platform.',
    );
  }

  @override
  Future<void> cancelByUniqueName(String uniqueName) async {
    _sender.send(_CancelByName(uniqueName));
  }

  @override
  Future<void> cancelByTag(String tag) async {
    _sender.send(_CancelByTag(tag));
  }

  @override
  Future<void> cancelAll() async {
    _sender.send(const _CancelAll());
  }

  @override
  Future<bool> isScheduledByUniqueName(String uniqueName) async {
    throw UnimplementedError(
      'No implementation found for workmanager on this platform.',
    );
  }

  @override
  Future<String> printScheduledTasks() async {
    throw UnimplementedError(
      'No implementation found for workmanager on this platform.',
    );
  }

  Future<void> dispose() async {
    // await _taskStreamController.close();
    // await _streamSubscription.cancel();
    _isolate.kill();
  }
}
