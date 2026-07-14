import 'package:flutter/material.dart';
import '../models/todo.dart';
import '../models/user.dart';

/// Service for managing Todo operations via ConnectRPC
/// 
/// This is a placeholder service that will be implemented once the
/// Rust server with ConnectRPC is ready. The actual implementation will
/// use the generated ConnectRPC client to communicate with the server.
class TodoService with ChangeNotifier {
  final String baseUrl;
  final User? user;

  TodoService({
    this.baseUrl = 'http://localhost:8080',
    this.user,
  });

  /// Fetch all todos for the current user
  /// TODO: Implement with ConnectRPC client once server is ready
  Future<List<Todo>> getTodos() async {
    debugPrint('Fetching todos from $baseUrl for user: ${user?.id}');
    // For now, return empty list as placeholder
    return [];
  }

  /// Create a new todo
  /// TODO: Implement with ConnectRPC client once server is ready
  Future<Todo> createTodo(CreateTodoRequest request) async {
    debugPrint('Creating todo: ${request.title}');
    // For now, create a local todo as placeholder
    return Todo(
      title: request.title,
      description: request.description,
      userId: user?.id ?? 'unknown',
    );
  }

  /// Update an existing todo
  /// TODO: Implement with ConnectRPC client once server is ready
  Future<Todo> updateTodo(UpdateTodoRequest request) async {
    debugPrint('Updating todo: ${request.id}');
    // For now, return a placeholder
    throw UnimplementedError('Update todo not implemented yet');
  }

  /// Delete a todo
  /// TODO: Implement with ConnectRPC client once server is ready
  Future<void> deleteTodo(String todoId) async {
    debugPrint('Deleting todo: $todoId');
    // For now, just print the action
  }

  /// Toggle completion status of a todo
  /// TODO: Implement with ConnectRPC client once server is ready
  Future<Todo> toggleTodoCompletion(String todoId, bool completed) async {
    debugPrint('Toggling todo completion: $todoId to $completed');
    // For now, return a placeholder
    throw UnimplementedError('Toggle todo completion not implemented yet');
  }

  /// Refresh the todo list
  Future<void> refresh() async {
    notifyListeners();
  }
}
