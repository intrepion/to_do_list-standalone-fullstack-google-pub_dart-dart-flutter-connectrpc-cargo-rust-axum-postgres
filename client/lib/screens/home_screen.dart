// Home Screen
// Displays the list of todos

import 'package:flutter/material.dart';

class HomeScreen extends StatelessWidget {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('My Todos'),
        actions: [
          // TODO: Add settings/logout button
          // IconButton(
          //   icon: const Icon(Icons.settings),
          //   onPressed: () => _showSettings(context),
          // ),
          // IconButton(
          //   icon: const Icon(Icons.logout),
          //   onPressed: () => _handleLogout(context),
          // ),
        ],
      ),
      body: const Center(
        child: Text('Todo list will appear here'),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          // TODO: Navigate to add todo screen
        },
        child: const Icon(Icons.add),
      ),
    );
  }
}
