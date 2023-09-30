import 'package:flutter/material.dart';
import 'package:gitgui/route/route.dart' as route;

class Config extends StatelessWidget {
  const Config({super.key});

  @override
  Widget build(BuildContext context) {
    return Center(
      child: ElevatedButton(
        onPressed: () async {
          Navigator.of(context).pop();
          var r = await Navigator.of(context).pushNamed(route.aboutPage);
          print(r);
        },
        child: const Text("Goto About page"),
      ),
    );
  }
}
