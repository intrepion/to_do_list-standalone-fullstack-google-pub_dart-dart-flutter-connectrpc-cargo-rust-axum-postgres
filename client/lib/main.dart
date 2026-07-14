import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

// TODO: Import generated ConnectRPC clients
// import 'package:client/generated/todo_service.connect.dart';

void main() {
  runApp(
    MultiProvider(
      providers: [
        // TODO: Add auth provider
        // ChangeNotifierProvider(create: (_) => AuthState()),
        // TODO: Add todo provider
        // ChangeNotifierProvider(create: (_) => TodoState()),
      ],
      child: const MyApp(),
    ),
  );
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Todo App',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      // TODO: Add named routes for navigation
      // routes: {
      //   '/': (context) => const SplashScreen(),
      //   '/login': (context) => const LoginScreen(),
      //   '/home': (context) => const HomeScreen(),
      // },
      home: const Scaffold(
        body: Center(
          child: Text('Todo App - Setup in Progress'),
        ),
      ),
    );
  }
}
