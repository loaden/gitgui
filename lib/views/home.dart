import 'dart:io';
import 'package:flutter/material.dart';
import 'package:file_picker/file_picker.dart';
import 'package:gitgui/bridge_api.dart';
import 'package:gitgui/route/route.dart' as route;
import 'package:gitgui/native.dart';

class Home extends StatefulWidget {
  const Home({super.key});

  @override
  State<Home> createState() => _HomeState();
}

class OverTextEditingController extends TextEditingController {
  List<DiffLine> lines;
  OverTextEditingController({required this.lines});

  @override
  TextSpan buildTextSpan(
      {required BuildContext context,
      TextStyle? style,
      required bool withComposing}) {
    return TextSpan(
      style: style,
      children: _diffTextSpans(),
    );
  }

  List<InlineSpan> _diffTextSpans() {
    return List.generate(lines.length, (index) {
      DiffLine item = lines[index];
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
}

class _HomeState extends State<Home> {
  final _diffController = OverTextEditingController(lines: []);

  @override
  void initState() {
    super.initState();

    if (Platform.isMacOS) {
      _rustOpenRepo();
    } else {
      _rustOpenDefaultRepo();
    }
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
                  Expanded(
                    child: ListView(
                      children: const [
                        Text("data"),
                        Text("data"),
                      ],
                    ),
                  ),
                  const SizedBox(height: 20),
                  ElevatedButton(
                    onPressed: () => _rustOpenRepo(),
                    child: const Text("Open Repo"),
                  ),
                  const SizedBox(height: 10),
                  ElevatedButton(
                    onPressed: () => _rustFetchStatus(),
                    child: const Text("Fetch Status"),
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
              child: TextField(
                controller: _diffController,
                readOnly: true,
                minLines: null,
                maxLines: null,
                expands: true,
                textAlign: TextAlign.start,
                textAlignVertical: TextAlignVertical.top,
                decoration: const InputDecoration(
                  labelText: 'Diff',
                  helperText: "内容显示区",
                  border: OutlineInputBorder(
                    borderRadius: BorderRadius.all(Radius.circular(8)),
                  ),
                ),
              ),
            ),
          ],
        ),
      ),
    );
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

  Future<void> _rustOpenDefaultRepo() async {
    api.openDefaultRepo();
    _rustGetDiff();
  }

  Future<void> _rustOpenRepo() async {
    String d = await api.getDefaultRepo();
    String? selectedDirectory = await FilePicker.platform.getDirectoryPath(
      dialogTitle: "选择源码库",
      initialDirectory: d,
    );
    if (selectedDirectory != null) {
      await api.setRepo(path: selectedDirectory);
      _rustGetDiff();
    }
  }

  Future<void> _rustFetchStatus() async {
    await api.fetchStatus();
    _rustGetDiff();
  }

  Future<void> _rustGetDiff() async {
    final diff = await api.getDiff();
    if (mounted) {
      setState(() {
        String text = "";
        for (var e in diff) {
          text += e.content;
        }
        _diffController.lines = diff;
        _diffController.text = text;
      });
    }
  }
}
