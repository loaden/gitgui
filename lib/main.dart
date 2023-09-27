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
      body: const Row(
        children: [
          Button(),
          Button(),
        ],
      ),
    );
  }
}

class Button extends StatefulWidget {
  const Button({super.key});

  @override
  State<Button> createState() => _ButtonState();
}

class _ButtonState extends State<Button> {
  bool _chcl = false;
  @override
  Widget build(BuildContext context) {
    return ElevatedButton(
      onPressed: _add,
      onLongPress: _changeColor,
      style: ElevatedButton.styleFrom(
          backgroundColor: _chcl ? Colors.orange : Colors.green),
      child: const Icon(Icons.add),
    );
  }

  // ignore: avoid_print
  void _add() => print('hello');

  void _changeColor() {
    setState(() {
      _chcl = !_chcl;
    });
  }
}
