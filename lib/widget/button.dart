import 'package:flutter/material.dart';

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
