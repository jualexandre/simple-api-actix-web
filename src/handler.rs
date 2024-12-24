use actix_web::{delete, get, patch, post, web, HttpResponse, Responder, Scope};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::{
    model::{AppState, QueryOptions, Todo, UpdateTodoSchema},
    response::{GenericResponse, TodoListResponse, SimpleTodoResponse, TodoData},
};

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope: Scope = web::scope("/api")
        .service(health_check_handler)
        .service(todos_list_handler)
        .service(create_todo_handler)
        .service(get_todo_handler)
        .service(edit_todo_handler)
        .service(delete_todo_handler);

    cfg.service(scope);
}

#[get("/health")]
async fn health_check_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust and Actix Web";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };

    HttpResponse::Ok().json(response_json)
}

#[get("/todos")]
async fn todos_list_handler(
    opts: web::Query<QueryOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let todos = data.todo_db.lock().unwrap();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let todos: Vec<Todo> = todos.clone().into_iter().skip(offset).take(limit).collect();
    let json_response = TodoListResponse {
        status: "success".to_string(),
        results: todos.len(),
        todos,
    };

    HttpResponse::Ok().json(json_response)
}

#[post("/todos")]
async fn create_todo_handler(
    mut body: web::Json<Todo>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut vec = data.todo_db.lock().unwrap();

    let todo = vec.iter().find(|todo| todo.title == body.title);

    if todo.is_some() {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Todo with title: '{}' already exists", body.title),
        };
        return HttpResponse::Conflict().json(error_response);
    }

    let uuid: Uuid = Uuid::new_v4();
    let datetime: DateTime<Utc> = Utc::now();

    body.id = Some(uuid.to_string());
    body.completed = Some(false);
    body.createdAt = Some(datetime);
    body.updatedAt = Some(datetime);

    let todo: Todo = body.to_owned();
    vec.push(body.into_inner());

    let json_response = SimpleTodoResponse {
        status: "success".to_string(),
        data: TodoData {todo}
    };

    HttpResponse::Created().json(json_response)
}

#[get("/todos/{id}")]
async fn get_todo_handler(
    path: web::Path<String>,
    data: web::Data<AppState>
) -> impl Responder {
    let vec = data.todo_db.lock().unwrap();

    let id: String = path.into_inner();
    let todo: Option<&Todo> = vec.iter().find(|todo| todo.id == Some(id.to_owned()));

    if todo.is_none() {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Todo with ID: {} not found", id),
        };
        return HttpResponse::NotFound().json(error_response);
    }

    let todo: &Todo = todo.unwrap();
    let json_response = SimpleTodoResponse {
        status: "success".to_string(),
        data: TodoData { todo: todo.clone() }
    };

    HttpResponse::Ok().json(json_response)
}

#[patch("/todos/{id}")]
async fn edit_todo_handler(
    path: web::Path<String>,
    body: web::Json<UpdateTodoSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut vec = data.todo_db.lock().unwrap();

    let id: String = path.into_inner();
    let todo: Option<&mut Todo> = vec.iter_mut().find(|todo| todo.id == Some(id.to_owned()));

    if todo.is_none() {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Todo with ID: {} not found", id)
        };
        return HttpResponse::NotFound().json(error_response);
    }

    let todo: &mut Todo = todo.unwrap();
    let datetime: DateTime<Utc> = Utc::now();
    let title: String = body.title.to_owned().unwrap_or(todo.title.to_owned());
    let content: String = body.content.to_owned().unwrap_or(todo.content.to_owned());
    let payload = Todo {
        id: todo.id.to_owned(),
        title: if !title.is_empty() {
            title
        } else {
            todo.title.to_owned()
        },
        content: if !content.is_empty() {
            content
        } else {
            todo.content.to_owned()
        },
        completed: if body.completed.is_some() {
            body.completed
        } else {
            todo.completed
        },
        createdAt: todo.createdAt,
        updatedAt: Some(datetime),
    };
    *todo = payload;

    let json_response = SimpleTodoResponse {
        status: "success".to_string(),
        data: TodoData { todo: todo.clone() },
    };

    HttpResponse::Ok().json(json_response)
}

#[delete("/todos/{id}")]
async fn delete_todo_handler(
    path: web::Path<String>,
    data: web::Data<AppState>
) -> impl Responder {
    let mut vec = data.todo_db.lock().unwrap();

    let id: String = path.into_inner();
    let todo: Option<&mut Todo> = vec.iter_mut().find(|todo| todo.id == Some(id.to_owned()));

    if todo.is_none() {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Todo with ID: {} not found", id),
        };
        return HttpResponse::NotFound().json(error_response);
    }

    vec.retain(|todo| todo.id != Some(id.to_owned()));

    HttpResponse::NoContent().finish()
}

