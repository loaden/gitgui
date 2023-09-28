import 'package:flutter/material.dart';
import 'package:gitgui/widget/button.dart';
import 'package:gitgui/route/route.dart' as route;

class Home extends StatelessWidget {
  const Home({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Hi')),
      body: Row(
        children: [
          Button(onPressed: (bool b) => _onBtn1(b, context)),
          Button(onPressed: _onBtn2),
        ],
      ),
    );
  }

  void _onBtn1(bool b, BuildContext context) {
    Navigator.of(context).pushNamed(route.aboutPage);
  }

  void _onBtn2(bool b) {
    print('2 $b');
  }
}
