import 'dart:io';
import 'dart:math';

import 'package:flutter_local_notifications/flutter_local_notifications.dart';

class NotificationService {
  static final FlutterLocalNotificationsPlugin _plugin =
      FlutterLocalNotificationsPlugin();

  static Future<void> init() async {
    const androidSettings = AndroidInitializationSettings(
      '@mipmap/ic_launcher',
    );
    const iosSettings = DarwinInitializationSettings(
      requestAlertPermission: true,
      requestBadgePermission: true,
      requestSoundPermission: true,
    );
    const macosSettings = DarwinInitializationSettings(
      requestAlertPermission: true,
      requestBadgePermission: true,
      requestSoundPermission: true,
    );

    const linuxSettings = LinuxInitializationSettings(
      defaultActionName: 'Stride',
    );

    const settings = InitializationSettings(
      android: androidSettings,
      iOS: iosSettings,
      macOS: macosSettings,
      linux: linuxSettings,
    );

    await _plugin.initialize(settings: settings);
  }

  static Future<void> show(String title, String? body) async {
    if (Platform.isAndroid) {
      await _plugin
          .resolvePlatformSpecificImplementation<
            AndroidFlutterLocalNotificationsPlugin
          >()!
          .requestNotificationsPermission();
    }

    final androidDetails = AndroidNotificationDetails(
      'stride_background',
      'Background notifications',
      channelDescription: 'Notifications about background activities.',
      importance: Importance.max,
      priority: Priority.high,
      ticker: 'Stride background notification',
    );

    final details = NotificationDetails(
      android: androidDetails,
      iOS: const DarwinNotificationDetails(),
      macOS: const DarwinNotificationDetails(),
      linux: const LinuxNotificationDetails(),
    );

    final uniqueId =
        DateTime.now().millisecondsSinceEpoch.remainder(100000) * 100 +
        Random().nextInt(100);

    await _plugin.show(
      id: uniqueId,
      title: title,
      body: body,
      notificationDetails: details,
    );
  }
}
