/// Model class representing a User
class User {
  final String id;
  final String email;
  final String? displayName;
  final String? photoUrl;
  final String? googleId;
  final String? accessToken;
  final String? refreshToken;
  final DateTime? tokenExpiry;

  User({
    required this.id,
    required this.email,
    this.displayName,
    this.photoUrl,
    this.googleId,
    this.accessToken,
    this.refreshToken,
    this.tokenExpiry,
  });

  /// Create a User from JSON
  factory User.fromJson(Map<String, dynamic> json) {
    return User(
      id: json['id'] as String,
      email: json['email'] as String,
      displayName: json['display_name'] as String?,
      photoUrl: json['photo_url'] as String?,
      googleId: json['google_id'] as String?,
      accessToken: json['access_token'] as String?,
      refreshToken: json['refresh_token'] as String?,
      tokenExpiry: json['token_expiry'] != null
          ? DateTime.parse(json['token_expiry'] as String)
          : null,
    );
  }

  /// Convert User to JSON
  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'email': email,
      'display_name': displayName,
      'photo_url': photoUrl,
      'google_id': googleId,
      'access_token': accessToken,
      'refresh_token': refreshToken,
      'token_expiry': tokenExpiry?.toIso8601String(),
    };
  }

  /// Check if the access token is expired
  bool get isTokenExpired {
    if (tokenExpiry == null) return true;
    return DateTime.now().isAfter(tokenExpiry!);
  }

  /// Create a copy with updated fields
  User copyWith({
    String? id,
    String? email,
    String? displayName,
    String? photoUrl,
    String? googleId,
    String? accessToken,
    String? refreshToken,
    DateTime? tokenExpiry,
  }) {
    return User(
      id: id ?? this.id,
      email: email ?? this.email,
      displayName: displayName ?? this.displayName,
      photoUrl: photoUrl ?? this.photoUrl,
      googleId: googleId ?? this.googleId,
      accessToken: accessToken ?? this.accessToken,
      refreshToken: refreshToken ?? this.refreshToken,
      tokenExpiry: tokenExpiry ?? this.tokenExpiry,
    );
  }

  @override
  String toString() {
    return 'User(id: $id, email: $email, displayName: $displayName)';
  }

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    return other is User && other.id == id;
  }

  @override
  int get hashCode {
    return id.hashCode;
  }
}
