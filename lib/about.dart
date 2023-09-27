import 'package:flutter/material.dart';

class About extends StatelessWidget {
  const About({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('About')),
      body: ElevatedButton(
        child: const Text('Back to home'),
        onPressed: () => {Navigator.pop(context)},
      ),
    );
  }
}
