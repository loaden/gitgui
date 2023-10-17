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
      body: Row(
        children: [
          ConstrainedBox(
            constraints: const BoxConstraints(
              minHeight: 50,
              minWidth: 150,
              maxHeight: double.infinity,
              maxWidth: 150,
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
          const SizedBox(width: 20),
          Column(
            children: [
              ConstrainedBox(
                constraints: const BoxConstraints(
                  minHeight: 60,
                  minWidth: 200,
                  maxHeight: double.infinity,
                  maxWidth: 300,
                ),
                child: const TextField(),
              ),
              const SizedBox(
                width: 180,
                child: TextField(),
              ),
              const SizedBox(height: 20),
              navRoute(context),
            ],
          )
        ],
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
