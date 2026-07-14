// User Model
// Data model for User information

class User {
  final String id;
  final String googleId;
  final String email;
  final String name;
  final DateTime createdAt;

  User({
    required this.id,
    required this.googleId,
    required this.email,
    required this.name,
    required this.createdAt,
  });

  // TODO: Add fromJson/toJson methods
  // TODO: Add copyWith method
  // TODO: Add equality and hashCode
}
