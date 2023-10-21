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
  List<DiffLine> _mainSpansList = [];

  Paint paint = Paint()
    ..color = Colors.blue
    ..style = PaintingStyle.stroke
    ..strokeCap = StrokeCap.round
    ..strokeWidth = 2.0;

  @override
  void initState() {
    super.initState();
    _rustAppRun();
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
                    onPressed: () => _rustFetchStatus(),
                    child: const Text("Fetch Status"),
                  ),
                  const SizedBox(height: 10),
                  ElevatedButton(
                    onPressed: () => _rustUpdateDiff(),
                    child: const Text("Update Diff"),
                  ),
                  const SizedBox(height: 10),
                  ElevatedButton(
                    onPressed: () => _rustGetDiff(),
                    child: const Text("Get Diff"),
                  ),
                  const SizedBox(height: 20),
                  _navRoute(context),
                ],
              ),
            ),
            const SizedBox(width: 10),
            Expanded(
              child: RichText(
                text: TextSpan(
                  style: TextStyle(background: paint),
                  children: _mainTextSpans(),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  List<InlineSpan> _mainTextSpans() {
    return List.generate(_mainSpansList.length, (index) {
      DiffLine item = _mainSpansList[index];
      return TextSpan(
        text: item.content,
        style: TextStyle(
          color: () {
            if (item.lineType == DiffLineType.Add) {
              return Colors.green;
            } else if (item.lineType == DiffLineType.Delete) {
              return Colors.red;
            } else if (item.lineType == DiffLineType.Header) {
              return Colors.blue;
            } else {
              return Colors.grey;
            }
          }(),
        ),
      );
    });
  }

  Row _navRoute(BuildContext context) {
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

  Future<void> _rustFetchStatus() async {
    await api.fetchStatus();
  }

  void _rustFetchStatus2() {
    api.fetchStatus();
  }

  Future<void> _rustUpdateDiff() async {
    await api.updateDiff();
  }

  Future<void> _rustGetDiff() async {
    final diff = await api.getDiff();
    if (mounted) {
      setState(() => _mainSpansList = diff);
    }
  }
}
