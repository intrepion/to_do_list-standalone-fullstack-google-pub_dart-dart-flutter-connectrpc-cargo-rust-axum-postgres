// Todo Provider
// State management for todos

import 'package:flutter/material.dart';
import '../models/todo.dart';

class TodoState with ChangeNotifier {
  List<Todo> _todos = [];
  bool _isLoading = false;
  String? _error;

  List<Todo> get todos => _todos;
  bool get isLoading => _isLoading;
  String? get error => _error;

  // TODO: Implement fetch todos
  // TODO: Implement create todo
  // TODO: Implement update todo
  // TODO: Implement delete todo
  // TODO: Implement filter by user

  void setTodos(List<Todo> todos) {
    _todos = todos;
    notifyListeners();
  }

  void addTodo(Todo todo) {
    _todos.add(todo);
    notifyListeners();
  }

  void updateTodo(Todo updatedTodo) {
    final index = _todos.indexWhere((t) => t.id == updatedTodo.id);
    if (index != -1) {
      _todos[index] = updatedTodo;
      notifyListeners();
    }
  }

  void removeTodo(String todoId) {
    _todos.removeWhere((t) => t.id == todoId);
    notifyListeners();
  }

  void setLoading(bool loading) {
    _isLoading = loading;
    notifyListeners();
  }

  void setError(String? error) {
    _error = error;
    notifyListeners();
  }
}
