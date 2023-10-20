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
                  children: List.generate(_mainSpansList.length, (index) {
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
                  }),
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
      setState(() => _mainSpansList = diff);
      print(_mainSpansList);
    }
  }
}
