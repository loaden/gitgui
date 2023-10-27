import 'package:flutter/material.dart';

class BoxContainer extends StatelessWidget {
  final String title;
  final Widget child;
  const BoxContainer({super.key, required this.title, required this.child});

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        Container(
          margin: const EdgeInsets.only(top: 25),
          decoration: BoxDecoration(
            border: Border.all(color: Theme.of(context).disabledColor),
            borderRadius: BorderRadius.circular(8.0),
          ),
          child: child,
        ),
        Positioned(
          left: 15,
          top: 15,
          child: Container(
            padding: const EdgeInsets.only(left: 5, right: 5),
            color: Theme.of(context).scaffoldBackgroundColor,
            child: Text(
              title,
              style: TextStyle(
                color: Theme.of(context).unselectedWidgetColor,
                fontSize: 14,
              ),
            ),
          ),
        ),
      ],
    );
  }
}
