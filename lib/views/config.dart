import 'package:flutter/material.dart';
import 'package:gitgui/route/route.dart' as route;

class Config extends StatelessWidget {
  const Config({super.key});

  @override
  Widget build(BuildContext context) {
    return Center(
      child: ElevatedButton(
        onPressed: () {
          Navigator.of(context).pop();
          Navigator.of(context).pushNamed(route.aboutPage);
        },
        child: const Text("Goto About page"),
      ),
    );
  }
}
