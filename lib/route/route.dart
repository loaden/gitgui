import 'package:flutter/material.dart';

import 'package:gitgui/views/about.dart';
import 'package:gitgui/views/home.dart';

// Route names
const String aboutPage = 'about';
const String homePage = 'home';

// Control function
Route<dynamic> controller(RouteSettings settings) {
  switch (settings.name) {
    case aboutPage:
      return MaterialPageRoute(builder: (context) => const About());
    case homePage:
      return MaterialPageRoute(builder: (context) => const Home());
    default:
      throw ('The route does not exist');
  }
}
