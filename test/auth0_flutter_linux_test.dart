import 'package:flutter_test/flutter_test.dart';
import 'package:auth0_flutter_linux/auth0_flutter_linux.dart';
import 'package:auth0_flutter_linux/auth0_flutter_linux_platform_interface.dart';
import 'package:auth0_flutter_linux/auth0_flutter_linux_method_channel.dart';
import 'package:plugin_platform_interface/plugin_platform_interface.dart';

class MockAuth0FlutterLinuxPlatform
    with MockPlatformInterfaceMixin
    implements Auth0FlutterLinuxPlatform {

  @override
  Future<String?> getPlatformVersion() => Future.value('42');
}

void main() {
  final Auth0FlutterLinuxPlatform initialPlatform = Auth0FlutterLinuxPlatform.instance;

  test('$MethodChannelAuth0FlutterLinux is the default instance', () {
    expect(initialPlatform, isInstanceOf<MethodChannelAuth0FlutterLinux>());
  });

  test('getPlatformVersion', () async {
    Auth0FlutterLinux auth0FlutterLinuxPlugin = Auth0FlutterLinux();
    MockAuth0FlutterLinuxPlatform fakePlatform = MockAuth0FlutterLinuxPlatform();
    Auth0FlutterLinuxPlatform.instance = fakePlatform;

    expect(await auth0FlutterLinuxPlugin.getPlatformVersion(), '42');
  });
}
