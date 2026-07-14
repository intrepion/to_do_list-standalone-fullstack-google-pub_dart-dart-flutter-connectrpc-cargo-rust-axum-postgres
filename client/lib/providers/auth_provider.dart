import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:google_sign_in/google_sign_in.dart';
import 'package:shared_preferences/shared_preferences.dart';
import '../models/user.dart';

/// Authentication state enum
enum AuthState {
  unknown,
  authenticated,
  unauthenticated,
  loading,
  error,
}

/// Provider for managing authentication state
class AuthProvider with ChangeNotifier {
  AuthState _authState = AuthState.unknown;
  User? _user;
  String? _errorMessage;

  static const String _prefsKeyUser = 'user';
  static const String _prefsKeyAccessToken = 'access_token';
  static const String _prefsKeyRefreshToken = 'refresh_token';
  static const String _prefsKeyTokenExpiry = 'token_expiry';

  final GoogleSignIn _googleSignIn = GoogleSignIn(
    scopes: [
      'email',
      'profile',
      'openid',
    ],
  );

  AuthState get authState => _authState;
  User? get user => _user;
  String? get errorMessage => _errorMessage;

  AuthProvider() {
    _loadFromPrefs();
  }

  /// Load authentication state from shared preferences
  Future<void> _loadFromPrefs() async {
    try {
      final prefs = await SharedPreferences.getInstance();
      final userJson = prefs.getString(_prefsKeyUser);
      final accessToken = prefs.getString(_prefsKeyAccessToken);
      final refreshToken = prefs.getString(_prefsKeyRefreshToken);
      final tokenExpiryString = prefs.getString(_prefsKeyTokenExpiry);

      if (userJson != null && accessToken != null) {
        final userData = Map<String, dynamic>.from(
          jsonDecode(userJson) as Map<String, dynamic>,
        );
        DateTime? tokenExpiry;
        if (tokenExpiryString != null) {
          tokenExpiry = DateTime.parse(tokenExpiryString);
        }

        _user = User.fromJson(userData).copyWith(
          accessToken: accessToken,
          refreshToken: refreshToken,
          tokenExpiry: tokenExpiry,
        );
        
        if (_user!.isTokenExpired) {
          _authState = AuthState.unauthenticated;
        } else {
          _authState = AuthState.authenticated;
        }
      } else {
        _authState = AuthState.unauthenticated;
      }
      notifyListeners();
    } catch (e) {
      _authState = AuthState.error;
      _errorMessage = 'Failed to load auth state: $e';
      notifyListeners();
    }
  }

  /// Sign in with Google
  Future<bool> signInWithGoogle() async {
    try {
      _authState = AuthState.loading;
      _errorMessage = null;
      notifyListeners();

      // Check if already signed in
      if (await _googleSignIn.isSignedIn()) {
        await _googleSignIn.signOut();
      }

      final googleUser = await _googleSignIn.signIn();
      if (googleUser == null) {
        _authState = AuthState.unauthenticated;
        _errorMessage = 'Sign in was cancelled';
        notifyListeners();
        return false;
      }

      final googleAuth = await googleUser.authentication;
      
      // Note: GoogleSignInAuthentication doesn't provide refreshToken or tokenExpiry
      // in the current version. For server-side auth, we'll use serverAuthCode
      final serverAuthCode = googleUser.serverAuthCode;
      
      final user = User(
        id: googleUser.id,
        email: googleUser.email,
        displayName: googleUser.displayName,
        photoUrl: googleUser.photoUrl,
        googleId: googleUser.id,
        accessToken: googleAuth.accessToken,
        // For web, we might get refresh token via server auth code exchange
        // For mobile, the google_sign_in plugin handles token refresh automatically
        refreshToken: serverAuthCode,
        // Set token expiry to 1 hour from now (typical Google token lifetime)
        tokenExpiry: DateTime.now().add(const Duration(hours: 1)),
      );

      _user = user;
      _authState = AuthState.authenticated;
      
      // Save to preferences
      await _saveToPrefs();
      
      notifyListeners();
      return true;
    } catch (e) {
      _authState = AuthState.error;
      _errorMessage = 'Failed to sign in: $e';
      notifyListeners();
      return false;
    }
  }

  /// Sign out
  Future<void> signOut() async {
    try {
      await _googleSignIn.signOut();
      _user = null;
      _authState = AuthState.unauthenticated;
      _errorMessage = null;
      
      // Clear preferences
      final prefs = await SharedPreferences.getInstance();
      await prefs.remove(_prefsKeyUser);
      await prefs.remove(_prefsKeyAccessToken);
      await prefs.remove(_prefsKeyRefreshToken);
      await prefs.remove(_prefsKeyTokenExpiry);
      
      notifyListeners();
    } catch (e) {
      _authState = AuthState.error;
      _errorMessage = 'Failed to sign out: $e';
      notifyListeners();
    }
  }

  /// Save current auth state to preferences
  Future<void> _saveToPrefs() async {
    if (_user == null) return;
    
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString(_prefsKeyUser, jsonEncode(_user!.toJson()));
    await prefs.setString(_prefsKeyAccessToken, _user!.accessToken ?? '');
    await prefs.setString(_prefsKeyRefreshToken, _user!.refreshToken ?? '');
    await prefs.setString(
      _prefsKeyTokenExpiry,
      _user!.tokenExpiry?.toIso8601String() ?? '',
    );
  }

  /// Get the current access token
  String? get accessToken => _user?.accessToken;

  /// Check if token is expired and refresh if needed
  Future<String?> getValidToken() async {
    if (_user == null) return null;
    
    if (_user!.isTokenExpired) {
      // TODO: Implement token refresh
      // For now, just return null to force re-authentication
      return null;
    }
    
    return _user!.accessToken;
  }

  /// Clear any error state
  void clearError() {
    _errorMessage = null;
    _authState = _user != null ? AuthState.authenticated : AuthState.unauthenticated;
    notifyListeners();
  }
}

// Helper function to encode User to JSON
dynamic jsonEncode(Object? value) {
  if (value is Map) {
    return value.map<String, dynamic>((key, value) => MapEntry(key.toString(), jsonEncode(value)));
  } else if (value is List) {
    return value.map((e) => jsonEncode(e)).toList();
  } else if (value is DateTime) {
    return value.toIso8601String();
  } else if (value is String || value is num || value is bool || value == null) {
    return value;
  } else {
    return value.toString();
  }
}
