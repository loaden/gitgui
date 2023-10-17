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
      appBar: AppBar(title: Text(rustHiText ?? '')),
      body: Padding(
        padding: const EdgeInsets.all(8.0),
        child: Row(
          children: [
            ConstrainedBox(
              constraints: const BoxConstraints(
                minHeight: 50,
                minWidth: 100,
                maxHeight: double.infinity,
                maxWidth: 200,
              ),
              child: const Column(
                children: [
                  TextField(),
                  TextField(),
                  TextField(),
                  TextField(),
                  TextField(),
                ],
              ),
            ),
            const SizedBox(width: 10),
            Expanded(
              child: Column(
                children: [
                  const TextField(),
                  const SizedBox(height: 20),
                  navRoute(context),
                ],
              ),
            )
          ],
        ),
      ),
    );
  }

  Row navRoute(BuildContext context) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        Button(
          onPressed: (b) => Navigator.of(context).pushNamed(route.configPage),
        ),
        const SizedBox(width: 10),
        Button(onPressed: (b) async {
          var r = await Navigator.of(context).pushNamed(route.aboutPage);
          print(r);
        }),
      ],
    );
  }

  Future<void> _callHelloFromRust() async {
    final receivedText = await api.helloFromRust(count: 2);
    if (mounted) setState(() => rustHiText = receivedText);
  }
}
