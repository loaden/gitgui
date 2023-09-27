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
      body: Row(
        children: [
          Button(
            onPressed: _onBtn1,
          ),
          Button(
            onPressed: _onBtn2,
          ),
        ],
      ),
    );
  }

  void _onBtn1(bool b) {
    print('1 $b');
  }

  void _onBtn2(bool b) {
    print('2 $b');
  }
}

class Button extends StatefulWidget {
  const Button({super.key, required this.onPressed});
  final ValueChanged<bool> onPressed;

  @override
  State<Button> createState() => _ButtonState();
}

class _ButtonState extends State<Button> {
  bool _chcl = false;
  @override
  Widget build(BuildContext context) {
    return ElevatedButton(
      onPressed: _press,
      onLongPress: _changeColor,
      style: ElevatedButton.styleFrom(
          backgroundColor: _chcl ? Colors.orange : Colors.green),
      child: const Icon(Icons.add),
    );
  }

  void _press() => widget.onPressed(_chcl);

  void _changeColor() {
    setState(() {
      _chcl = !_chcl;
    });
  }
}
