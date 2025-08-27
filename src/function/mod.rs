use crate::database::{Database, DatabaseContext};

pub fn add_todo(db: &mut DatabaseContext, todo: Database) {
    db.insert(todo);
}
