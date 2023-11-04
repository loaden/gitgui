import 'package:flutter/material.dart';
import 'package:gitgui/route.dart' as route;

class Config extends StatelessWidget {
  const Config({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Config Page')),
      body: Row(
        children: [
          ElevatedButton(
            onPressed: () async {
              var r = await route.go(context, route.aboutPage);
              print(r);
            },
            child: const Text("Goto About page"),
          ),
          const SizedBox(
            width: 10,
          ),
          ElevatedButton(
            onPressed: () => route.pop(context),
            child: const Text("Go Back"),
          ),
        ],
      ),
    );
  }
}
