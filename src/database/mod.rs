pub struct Database {
    pub todos: String,
    pub state: bool,
}
pub struct DatabaseContext {
    todos: Vec<Database>,
}

impl DatabaseContext {
    pub fn new() -> Self {
        DatabaseContext { todos: Vec::new() }
    }

    pub fn insert(&mut self, todo: Database) {
        self.todos.push(todo);
    }
    pub fn delete(&mut self, index: usize) {
        self.todos.remove(index);
    }
    pub fn mark(&mut self, index: usize) {
        if let Some(todo) = self.todos.get_mut(index) {
            todo.state = true;
        }
    }
    pub fn list(&self) -> &Vec<Database> {
        &self.todos
    }
}
