import 'package:flutter/material.dart';
import 'package:gitgui/route/route.dart' as route;

class About extends StatelessWidget {
  const About({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('About')),
      body: ElevatedButton(
        child: const Text('Back to home'),
        onPressed: () => Navigator.of(context).pushNamed(route.homePage),
      ),
    );
  }
}
