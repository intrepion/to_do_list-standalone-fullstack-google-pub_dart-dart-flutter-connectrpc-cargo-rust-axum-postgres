// Todo Model
// Data model for Todo items

class Todo {
  final String id;
  final String title;
  final String description;
  final bool completed;
  final DateTime createdAt;
  final DateTime updatedAt;
  final String userId;

  Todo({
    required this.id,
    required this.title,
    required this.description,
    required this.completed,
    required this.createdAt,
    required this.updatedAt,
    required this.userId,
  });

  // TODO: Add fromJson/toJson methods
  // TODO: Add copyWith method
  // TODO: Add equality and hashCode
}
