import 'package:auth0_flutter_platform_interface/auth0_flutter_platform_interface.dart';
import 'package:flutter/services.dart';

class Auth0FlutterWebAuthLinux extends Auth0FlutterWebAuthPlatform {
  static void registerWith() {
    Auth0FlutterWebAuthPlatform.instance = Auth0FlutterWebAuthLinux();
  }

  @override
  Future<Credentials> login(WebAuthRequest request) async {
    try {
      final options = request.options;
      // final parameters =
      //     options.parameters.entries.map((e) => (e.key, e.value)).toList();
      throw UnimplementedError();
    } catch (e) {
      if (e is WebAuthenticationException) {
        rethrow;
      }
      if (e is PlatformException) {
        throw WebAuthenticationException(
          e.code,
          e.message ?? 'Unknown error occurred',
          e.details,
        );
      }
      throw WebAuthenticationException('UNKNOWN_ERROR', e.toString(), {});
    }
  }
}
