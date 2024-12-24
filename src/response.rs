use serde::Serialize;
use crate::model::Todo;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct TodoData {
    pub todo: Todo,
}

#[derive(Debug, Serialize)]
pub struct SimpleTodoResponse {
    pub status: String,
    pub data: TodoData,
}

#[derive(Debug, Serialize)]
pub struct TodoListResponse {
    pub status: String,
    pub results: usize,
    pub todos: Vec<Todo>,
}
