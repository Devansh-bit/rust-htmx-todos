use crate::Todo;
use askama::Template;

#[derive(Template)]
#[template(path = "base.html")]
pub struct Base;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

#[derive(Template)]
#[template(path = "stream.html")]
pub struct StreamTemplate;

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
