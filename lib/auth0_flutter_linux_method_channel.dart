import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';

import 'auth0_flutter_linux_platform_interface.dart';

/// An implementation of [Auth0FlutterLinuxPlatform] that uses method channels.
class MethodChannelAuth0FlutterLinux extends Auth0FlutterLinuxPlatform {
  /// The method channel used to interact with the native platform.
  @visibleForTesting
  final methodChannel = const MethodChannel('auth0_flutter_linux');

  @override
  Future<String?> getPlatformVersion() async {
    final version = await methodChannel.invokeMethod<String>('getPlatformVersion');
    return version;
  }
}
