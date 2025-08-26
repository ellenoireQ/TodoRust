pub struct DatabaseContext{
    todos: Vec<String>
}

impl DatabaseContext{
    pub fn new() -> Self{
        DatabaseContext { todos: Vec::new() }
    }

    pub fn insert(&mut self, todo: String){
        self.todos.push(todo);
    }
    pub fn list(&self) -> &Vec<String>{
        &self.todos
    }
}