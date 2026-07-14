// API Client
// Base client for making API requests to the ConnectRPC server

import 'package:http/http.dart' as http;

class ApiClient {
  static const String _baseUrl = 'http://localhost:8080';
  
  final http.Client _client;
  String? _authToken;

  ApiClient({http.Client? client}) : _client = client ?? http.Client();

  void setAuthToken(String token) {
    _authToken = token;
  }

  void clearAuthToken() {
    _authToken = null;
  }

  // TODO: Add methods for making ConnectRPC calls
  // TODO: Add request interceptors for JWT injection
  // TODO: Add error handling

  Future<void> close() async {
    _client.close();
  }
}
