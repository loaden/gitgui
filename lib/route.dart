import 'package:flutter/material.dart';

import 'package:gitgui/views/about.dart';
import 'package:gitgui/views/home.dart';
import 'package:gitgui/views/config.dart';

// Route names
const String aboutPage = 'about';
const String homePage = 'home';
const String configPage = 'config';

// Control function
Route<dynamic> controller(RouteSettings settings) {
  switch (settings.name) {
    case aboutPage:
      return MaterialPageRoute(
          builder: (context) => const About(name: 'Lucky'));
    case homePage:
      return MaterialPageRoute(builder: (context) => const Home());
    case configPage:
      return MaterialPageRoute(builder: (context) => const Config());
    default:
      throw ('The route does not exist');
  }
}

// Helpers
Future<dynamic> go(BuildContext context, String page) {
  return Navigator.of(context).pushNamed(page);
}

void pop(BuildContext context, {String? info}) {
  return Navigator.of(context).pop(info);
}
