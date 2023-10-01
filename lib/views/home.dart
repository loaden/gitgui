import 'package:flutter/material.dart';
import 'package:gitgui/widget/button.dart';
import 'package:gitgui/route/route.dart' as route;
import 'package:gitgui/ffi.dart';

class Home extends StatelessWidget {
  const Home({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: ffiString()),
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
}

Widget ffiString() {
  return FutureBuilder<List<dynamic>>(
    future: Future.wait([api.helloFromRust(count: 2)]),
    builder: (context, snap) {
      final data = snap.data;
      if (data == null) {
        return const Text("Loading");
      }

      return Text('${data[0]}');
    },
  );
}
