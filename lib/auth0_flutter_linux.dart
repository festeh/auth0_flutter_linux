import 'dart:async';

import 'package:auth0_flutter_platform_interface/auth0_flutter_platform_interface.dart';
import 'package:flutter/foundation.dart';

import 'src/web_authentication.dart';
import 'auth0_flutter_linux_platform_interface.dart';

class Auth0FlutterLinux {
  Future<String?> getPlatformVersion() {
    return Auth0FlutterLinuxPlatform.instance.getPlatformVersion();
  }

  static void registerWith() {
    // Only register the plugin if we're on Linux
    if (!kIsWeb && defaultTargetPlatform == TargetPlatform.linux) {
      Auth0FlutterWebAuthLinux.registerWith();
    }
  }
}
