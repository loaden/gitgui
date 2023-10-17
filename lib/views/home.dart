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
  String? _rustHiText;
  final TextEditingController _mainController = TextEditingController();

  @override
  void initState() {
    super.initState();
    _callFromRust();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text(_rustHiText ?? 'Gitgui')),
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
                  navRoute(context),
                ],
              ),
            ),
            const SizedBox(width: 10),
            Expanded(
              child: TextField(
                controller: _mainController,
                readOnly: true,
                minLines: null,
                maxLines: null,
                expands: true,
                decoration: const InputDecoration(
                  labelText: 'Diff',
                  helperText: "内容显示区",
                  border: OutlineInputBorder(
                    borderRadius: BorderRadius.all(Radius.circular(8)),
                  ),
                ),
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

  Future<void> _callFromRust() async {
    final receivedText = await api.helloFromRust(count: 2);
    final diff = await api.getDiff();
    if (mounted) {
      setState(() {
        _mainController.text = diff;
        _rustHiText = receivedText;
      });
    }
  }
}
