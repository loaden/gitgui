import 'package:flutter/material.dart';
import 'package:gitgui/route.dart' as route;

class About extends StatelessWidget {
  const About({super.key, required this.name});
  final String name;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('About $name')),
      body: Center(
        child: ElevatedButton(
          child: const Text('Go Back'),
          onPressed: () => route.pop(context, 'pop from about page'),
        ),
      ),
    );
  }
}
