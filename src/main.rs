pub mod database;
mod function;
use std::io;

use crate::{database::DatabaseContext, function::{add_todo, list_todos}};
fn main(){
    let mut db = DatabaseContext::new();
    let mut buffer = String::new();
    println!("Enter a todo item:");
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    let todo = buffer.trim();
    add_todo(&mut db, todo.to_string());

    list_todos(&db);
}