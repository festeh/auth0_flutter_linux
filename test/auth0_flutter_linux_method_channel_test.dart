import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:auth0_flutter_linux/auth0_flutter_linux_method_channel.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  MethodChannelAuth0FlutterLinux platform = MethodChannelAuth0FlutterLinux();
  const MethodChannel channel = MethodChannel('auth0_flutter_linux');

  setUp(() {
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger.setMockMethodCallHandler(
      channel,
      (MethodCall methodCall) async {
        return '42';
      },
    );
  });

  tearDown(() {
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger.setMockMethodCallHandler(channel, null);
  });

  test('getPlatformVersion', () async {
    expect(await platform.getPlatformVersion(), '42');
  });
}
