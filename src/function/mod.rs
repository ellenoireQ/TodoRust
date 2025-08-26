use crate::database::DatabaseContext;

pub fn add_todo(db: &mut DatabaseContext, todo: String) {
    db.insert(todo);
}

pub fn list_todos(db: &DatabaseContext) {
    for (index, todo) in db.list().iter().enumerate() {
        println!("List Today: \n {}.{}", index + 1 ,todo);
    }
}
