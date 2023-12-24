use crate::{Todo, users::User};
use askama::Template;

#[derive(Template)]
#[template(path = "base.html")]
pub struct Base;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

#[derive(Template)]
#[template(path = "logged_in.html")]
pub struct UserLoggedIn {
    pub user: User
}

#[derive(Template)]
#[template(path = "todos.html")]
pub struct Records {
    pub todos: Vec<Todo>,
}

#[derive(Template)]
#[template(path = "todo.html")]
pub struct TodoNewTemplate {
    pub todo: Todo,
}
