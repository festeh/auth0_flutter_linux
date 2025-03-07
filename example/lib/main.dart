import 'package:flutter/material.dart';
import 'dart:async';
import 'dart:io';

import 'package:flutter/services.dart';
import 'package:auth0_flutter/auth0_flutter.dart';
import 'package:auth0_flutter_linux/auth0_flutter_linux.dart';
import 'package:auth0_flutter_linux/src/rust/frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_io.dart';

Future<void> main() async {
  // Print current working directory
  print('Current working directory: ${Directory.current.path}');
  var config = ExternalLibraryLoaderConfig(
    stem: 'auth0_flutter_linux',
    ioDirectory: 'lib/',
    webPrefix: 'pkg/',
  );
  await RustLib.init(
    externalLibrary: await loadExternalLibrary(config),
  );
  Auth0FlutterLinux.registerWith();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  String _platformVersion = 'Unknown';
  final _auth0FlutterLinuxPlugin = Auth0FlutterLinux();

  String status = "Not Authenticated";
  late Auth0 auth0;

  @override
  void initState() {
    super.initState();
    initPlatformState();
    auth0 = Auth0('YOUR_AUTH0_DOMAIN', 'YOUR_AUTH0_CLIENT_ID');
    login();
  }

  Future<void> login() async {
    setState(() {
      status = 'Attempting login...';
    });

    try {
      await auth0.webAuthentication().login();
      setState(() {
        status = 'Logged in successfully';
      });
    } catch (e) {
      print(e);
      setState(() {
        status = 'Error: ${e.toString()}';
      });
    }
  }

  // Platform messages are asynchronous, so we initialize in an async method.
  Future<void> initPlatformState() async {
    String platformVersion;
    // Platform messages may fail, so we use a try/catch PlatformException.
    // We also handle the message potentially returning null.
    try {
      platformVersion = await _auth0FlutterLinuxPlugin.getPlatformVersion() ??
          'Unknown platform version';
    } on PlatformException {
      platformVersion = 'Failed to get platform version.';
    }

    // If the widget was removed from the tree while the asynchronous platform
    // message was in flight, we want to discard the reply rather than calling
    // setState to update our non-existent appearance.
    if (!mounted) return;

    setState(() {
      _platformVersion = platformVersion;
    });
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: Text('Auth status: $status'),
        ),
        body: Center(
          child: Text('Running on: $_platformVersion\n'),
        ),
      ),
    );
  }
}
