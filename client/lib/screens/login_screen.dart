// Login Screen
// Handles Google Sign In

import 'package:flutter/material.dart';

class LoginScreen extends StatelessWidget {
  const LoginScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Login')),
      body: const Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Text('Welcome to Todo App'),
            SizedBox(height: 20),
            // TODO: Add Google Sign In button
            // ElevatedButton.icon(
            //   onPressed: () => _handleGoogleSignIn(context),
            //   icon: Icon(Icons.g_mobiledata),
            //   label: Text('Sign in with Google'),
            // ),
          ],
        ),
      ),
    );
  }
}
