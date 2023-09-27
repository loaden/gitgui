import 'package:flutter/material.dart';

void main() {
  runApp(
    MaterialApp(
      theme: ThemeData.dark(),
      home: const Home(),
    ),
  );
}

class Home extends StatelessWidget {
  const Home({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Hi')),
      body: ElevatedButton(
        onPressed: _add,
        child: const Icon(Icons.add),
      ),
    );
  }

  // ignore: avoid_print
  void _add() => print('hello');
}
