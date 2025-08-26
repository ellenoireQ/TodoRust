pub mod database;
mod function;

use crate::function::{add_todo, list_todos};
fn main() {
    // inferred at database
    let mut db = database::DatabaseContext::new();

    let mut state = true;
    while state {
        let mut buffer = String::new();
        println!("Please input Command add|delete|exit");
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Didn't expected the answer");

        let input = buffer.trim();

        match input {
            /*          *
             *  Add     *
             *          */
            "add" => {
                let mut buffer_inpt = String::new();
                std::io::stdin().read_line(&mut buffer_inpt).ok();
                add_todo(&mut db, buffer_inpt);
            }
            "delete" => {
                let db_list = db.list();
                for (index, items) in db_list.iter().enumerate() {
                    println!("{}", items);
                }
                db.delete(0);
            }
            "exit" => {
                state = false;
            }
            _ => {
                println!("You didnt inputted program, GETTT OUTTTT!!!");
                state = false;
            }
        }
    }
    list_todos(&db);
}
