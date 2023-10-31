import 'dart:io';
import 'package:flutter/material.dart';
import 'package:file_picker/file_picker.dart';
import 'package:gitgui/bridge_api.dart';
import 'package:gitgui/route/route.dart' as route;
import 'package:gitgui/native.dart';
import 'package:gitgui/widget/box_container.dart';

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
  final _commitMsgController = TextEditingController();
  List<String> _statusItems = [];
  List<String> _indexItems = [];
  int _statusSelect = 0;
  int _indexSelect = -1;

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
                minWidth: 200,
                maxHeight: double.infinity,
                maxWidth: 300,
              ),
              child: Column(
                children: [
                  TextField(
                    decoration: const InputDecoration(
                      labelText: 'Commit...',
                      prefixIcon: Icon(Icons.textsms_outlined),
                      border: OutlineInputBorder(
                        borderRadius: BorderRadius.all(Radius.circular(8)),
                      ),
                    ),
                    controller: _commitMsgController,
                    onSubmitted: (value) async {
                      bool r = await api.commit(msg: value);
                      if (r) {
                        _commitMsgController.clear();
                        _rustUpdate();
                      }
                    },
                  ),
                  Expanded(
                    child: BoxContainer(
                      title: "Status",
                      child: statusListView(),
                    ),
                  ),
                  Expanded(
                    child: BoxContainer(
                      title: "Index",
                      child: indexListView(),
                    ),
                  ),
                  const SizedBox(height: 20),
                  ElevatedButton(
                    onPressed: () => _rustOpenRepo(),
                    child: const Text("Open Repo"),
                  ),
                  const SizedBox(height: 10),
                  ElevatedButton(
                    onPressed: () => _rustUpdate(),
                    child: const Text("Update"),
                  ),
                  const SizedBox(height: 10),
                  ElevatedButton(
                    onPressed: () => _rustGetData(),
                    child: const Text("Get Diff"),
                  ),
                  const SizedBox(height: 20),
                  navRoute(context),
                  const SizedBox(height: 50),
                ],
              ),
            ),
            const SizedBox(width: 10),
            Expanded(
              child: diffTextField(),
            ),
          ],
        ),
      ),
    );
  }

  ListView statusListView() {
    return ListView(
      padding: const EdgeInsets.all(10),
      itemExtent: 36,
      children: List.generate(_statusItems.length, (index) {
        var item = _statusItems[index];
        return ListTile(
          title: Text(item),
          dense: true,
          trailing: _statusSelect == index
              ? const Icon(Icons.keyboard_arrow_right)
              : null,
          selected: _statusSelect == index,
          contentPadding: const EdgeInsets.symmetric(horizontal: 5.0),
          onTap: () {
            _statusSelect = index;
            _indexSelect = -1;
            _rustSetSelection(index, false);
          },
          onLongPress: () async {
            await api.indexUpdate();
            _rustSetSelection(index, false);
            _rustUpdate();
          },
        );
      }),
    );
  }

  ListView indexListView() {
    return ListView(
      padding: const EdgeInsets.all(10),
      itemExtent: 36,
      children: List.generate(_indexItems.length, (index) {
        var item = _indexItems[index];
        return ListTile(
          title: Text(item),
          dense: true,
          trailing: _indexSelect == index
              ? const Icon(Icons.keyboard_arrow_right_outlined)
              : null,
          selected: _indexSelect == index,
          contentPadding: const EdgeInsets.symmetric(horizontal: 5.0),
          onTap: () {
            _indexSelect = index;
            _statusSelect = -1;
            _rustSetSelection(index, true);
          },
          onLongPress: () async {
            await api.indexUpdate();
            _rustSetSelection(index, true);
            _rustUpdate();
          },
        );
      }),
    );
  }

  TextField diffTextField() {
    return TextField(
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

  Future<void> _rustOpenDefaultRepo() async {
    api.openDefaultRepo();
    _rustGetData();
  }

  Future<void> _rustOpenRepo() async {
    String d = await api.getDefaultRepo();
    String? selectedDirectory = await FilePicker.platform.getDirectoryPath(
      dialogTitle: "选择源码库",
      initialDirectory: d,
    );
    if (selectedDirectory != null) {
      await api.setRepo(path: selectedDirectory);
      _rustGetData();
    }
  }

  Future<void> _rustUpdate() async {
    await api.update();
    _rustGetData();
  }

  Future<void> _rustSetSelection(int index, bool stage) async {
    await api.setSelection(index: index, stage: stage);
    _rustGetData();
  }

  Future<void> _rustGetData() async {
    var s = api.getStatusItems();
    var i = api.getIndexItems();
    var d = api.getDiff();
    var result = await Future.wait([s, i, d]);
    var status = result[0] as List<String>;
    var index = result[1] as List<String>;
    var diff = result[2] as List<DiffLine>;
    if (mounted) {
      setState(() {
        String text = "";
        for (var e in diff) {
          text += e.content;
        }
        _diffController.lines = diff;
        _diffController.text = text;
        _statusItems = status;
        _indexItems = index;
      });
    }
  }
}
