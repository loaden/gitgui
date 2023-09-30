import 'package:flutter/material.dart';

class About extends StatelessWidget {
  const About({super.key, required this.name});
  final String name;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('About $name')),
      body: Center(
        child: ElevatedButton(
          child: const Text('Back to home'),
          onPressed: () => Navigator.of(context).pop('hi'),
        ),
      ),
    );
  }
}
