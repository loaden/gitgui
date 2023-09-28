import 'package:flutter/material.dart';
import 'package:gitgui/route/route.dart' as route;

void main() {
  runApp(const App());
}

class App extends StatelessWidget {
  const App({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme: ThemeData.dark(),
      onGenerateRoute: route.controller,
      initialRoute: route.homePage,
    );
  }
}
