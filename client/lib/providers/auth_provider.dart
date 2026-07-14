// Auth Provider
// State management for authentication

import 'package:flutter/material.dart';
import '../models/user.dart';

class AuthState with ChangeNotifier {
  User? _user;
  String? _jwtToken;
  bool _isLoading = false;
  bool _isAuthenticated = false;

  User? get user => _user;
  String? get jwtToken => _jwtToken;
  bool get isLoading => _isLoading;
  bool get isAuthenticated => _isAuthenticated;

  // TODO: Implement login method
  // TODO: Implement logout method
  // TODO: Implement token refresh
  // TODO: Implement auto-login from stored token
  
  void setUser(User user, String token) {
    _user = user;
    _jwtToken = token;
    _isAuthenticated = true;
    notifyListeners();
  }

  void clearUser() {
    _user = null;
    _jwtToken = null;
    _isAuthenticated = false;
    notifyListeners();
  }

  void setLoading(bool loading) {
    _isLoading = loading;
    notifyListeners();
  }
}
