// Authentication Service
// Handles Google OAuth authentication and JWT management

import 'package:google_sign_in/google_sign_in.dart';

class AuthService {
  final GoogleSignIn _googleSignIn;
  
  AuthService() : _googleSignIn = GoogleSignIn();

  // TODO: Implement Google Sign In
  // TODO: Implement token exchange with server
  // TODO: Implement JWT storage and retrieval
  // TODO: Implement logout functionality
  
  Future<bool> isSignedIn() async {
    return _googleSignIn.isSignedIn();
  }

  // TODO: Add more auth methods
}
