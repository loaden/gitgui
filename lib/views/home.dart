import 'dart:io';

import 'package:flutter/material.dart';
import 'package:gitgui/bridge_api.dart';
import 'package:gitgui/route/route.dart' as route;
import 'package:gitgui/native.dart';

class Home extends StatefulWidget {
  const Home({super.key});

  @override
  State<Home> createState() => _HomeState();
}

class _HomeState extends State<Home> {
  final List<InlineSpan> _mainTextSpans = <InlineSpan>[];

  @override
  void initState() {
    super.initState();

    _rustAppRun();
    sleep(Duration.zero);
    _rustGetDiff();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Gitgui')),
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
              child: Column(
                children: [
                  const TextField(),
                  const TextField(),
                  const TextField(),
                  const TextField(),
                  const TextField(),
                  const SizedBox(height: 20),
                  ElevatedButton(
                    onPressed: () => _rustGetDiff(),
                    child: const Text("Update"),
                  ),
                  const SizedBox(height: 20),
                  navRoute(context),
                ],
              ),
            ),
            const SizedBox(width: 10),
            Expanded(
              child: RichText(
                text: TextSpan(
                  children: List.from(_mainTextSpans),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Row navRoute(BuildContext context) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        ElevatedButton(
          onPressed: () async {
            var r = await Navigator.of(context).pushNamed(route.aboutPage);
            print(r);
          },
          child: const Text('About'),
        ),
      ],
    );
  }

  Future<void> _rustAppRun() async {
    await api.appRun();
  }

  Future<void> _rustGetDiff() async {
    final diff = await api.getDiff();
    if (mounted) {
      if (diff.isNotEmpty) _mainTextSpans.clear();
      setState(() {
        for (var e in diff) {
          _mainTextSpans.add(
            TextSpan(
              text: e.content.toString(),
              style: TextStyle(
                color: () {
                  if (e.lineType == DiffLineType.Add) {
                    return Colors.green;
                  } else if (e.lineType == DiffLineType.Delete) {
                    return Colors.red;
                  } else if (e.lineType == DiffLineType.Header) {
                    return Colors.blue;
                  } else {
                    return Colors.grey;
                  }
                }(),
              ),
            ),
          );
        }
      });
      print(_mainTextSpans);
    }
  }
}
