import 'package:uuid/uuid.dart';

/// Model class representing a Todo item
class Todo {
  final String id;
  final String title;
  final String? description;
  final bool completed;
  final DateTime createdAt;
  final DateTime updatedAt;
  final String userId;

  Todo({
    String? id,
    required this.title,
    this.description,
    this.completed = false,
    DateTime? createdAt,
    DateTime? updatedAt,
    required this.userId,
  })  : id = id ?? const Uuid().v4(),
        createdAt = createdAt ?? DateTime.now(),
        updatedAt = updatedAt ?? DateTime.now();

  /// Create a Todo from JSON
  factory Todo.fromJson(Map<String, dynamic> json) {
    return Todo(
      id: json['id'] as String?,
      title: json['title'] as String,
      description: json['description'] as String?,
      completed: json['completed'] as bool? ?? false,
      createdAt: json['created_at'] != null
          ? DateTime.parse(json['created_at'] as String)
          : null,
      updatedAt: json['updated_at'] != null
          ? DateTime.parse(json['updated_at'] as String)
          : null,
      userId: json['user_id'] as String,
    );
  }

  /// Convert Todo to JSON
  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'title': title,
      'description': description,
      'completed': completed,
      'created_at': createdAt.toIso8601String(),
      'updated_at': updatedAt.toIso8601String(),
      'user_id': userId,
    };
  }

  /// Create a copy with updated fields
  Todo copyWith({
    String? id,
    String? title,
    String? description,
    bool? completed,
    DateTime? createdAt,
    DateTime? updatedAt,
    String? userId,
  }) {
    return Todo(
      id: id ?? this.id,
      title: title ?? this.title,
      description: description ?? this.description,
      completed: completed ?? this.completed,
      createdAt: createdAt ?? this.createdAt,
      updatedAt: updatedAt ?? this.updatedAt,
      userId: userId ?? this.userId,
    );
  }

  @override
  String toString() {
    return 'Todo(id: $id, title: $title, completed: $completed, userId: $userId)';
  }

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    return other is Todo && other.id == id;
  }

  @override
  int get hashCode {
    return id.hashCode;
  }
}

/// Request object for creating a new Todo
class CreateTodoRequest {
  final String title;
  final String? description;

  CreateTodoRequest({
    required this.title,
    this.description,
  });

  Map<String, dynamic> toJson() {
    return {
      'title': title,
      'description': description,
    };
  }
}

/// Request object for updating a Todo
class UpdateTodoRequest {
  final String id;
  final String? title;
  final String? description;
  final bool? completed;

  UpdateTodoRequest({
    required this.id,
    this.title,
    this.description,
    this.completed,
  });

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> map = {};
    if (title != null) map['title'] = title;
    if (description != null) map['description'] = description;
    if (completed != null) map['completed'] = completed;
    return map;
  }
}
