import 'package:plugin_platform_interface/plugin_platform_interface.dart';

import 'auth0_flutter_linux_method_channel.dart';

abstract class Auth0FlutterLinuxPlatform extends PlatformInterface {
  /// Constructs a Auth0FlutterLinuxPlatform.
  Auth0FlutterLinuxPlatform() : super(token: _token);

  static final Object _token = Object();

  static Auth0FlutterLinuxPlatform _instance = MethodChannelAuth0FlutterLinux();

  /// The default instance of [Auth0FlutterLinuxPlatform] to use.
  ///
  /// Defaults to [MethodChannelAuth0FlutterLinux].
  static Auth0FlutterLinuxPlatform get instance => _instance;

  /// Platform-specific implementations should set this with their own
  /// platform-specific class that extends [Auth0FlutterLinuxPlatform] when
  /// they register themselves.
  static set instance(Auth0FlutterLinuxPlatform instance) {
    PlatformInterface.verifyToken(instance, _token);
    _instance = instance;
  }

  Future<String?> getPlatformVersion() {
    throw UnimplementedError('platformVersion() has not been implemented.');
  }
}
