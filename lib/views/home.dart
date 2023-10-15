import 'package:flutter/material.dart';
import 'package:gitgui/widget/button.dart';
import 'package:gitgui/route/route.dart' as route;
import 'package:gitgui/native.dart';

class Home extends StatefulWidget {
  const Home({super.key});

  @override
  State<Home> createState() => _HomeState();
}

class _HomeState extends State<Home> {
  String? rustHiText;

  @override
  void initState() {
    super.initState();
    _callHelloFromRust();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text(rustHiText!)),
      body: Center(
        child: Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Button(onPressed: (b) => _onBtn(b, context)),
            const SizedBox(width: 10),
            Button(onPressed: (b) async {
              var r = await Navigator.of(context).pushNamed(route.aboutPage);
              print(r);
            }),
          ],
        ),
      ),
    );
  }

  void _onBtn(bool b, BuildContext context) {
    Navigator.of(context).pushNamed(route.configPage);
  }

  Future<void> _callHelloFromRust() async {
    final receivedText = await api.helloFromRust(count: 2);
    if (mounted) setState(() => rustHiText = receivedText);
  }
}
