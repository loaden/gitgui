import 'package:flutter/material.dart';

void main() => runApp(const Home());

class Home extends StatelessWidget {
  const Home({super.key});

  @override
  Widget build(BuildContext context) {
    return const Text(
      'Hello World!',
      textDirection: TextDirection.ltr,
    );
  }
}
