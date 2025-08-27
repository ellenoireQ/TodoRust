pub mod database;
mod function;
pub mod table;
use comfy_table::Table;

use crate::{
    function::{add_todo, list_todos},
    table::TableData,
};
fn main() {
    // inferred at database
    let mut db = database::DatabaseContext::new();

    let mut state = true;
    while state {
        print!("\x1B[2J\x1B[1;1H");
        let table_datas = &db.list();
        table::build_table(table_datas);
        let mut buffer = String::new();
        let mut bufferDelete = String::new();
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
                print!("\x1B[2J\x1B[1;1H");
                let table_datas = &db.list();
                table::build_table(table_datas);
                println!("Input your task");
                let mut buffer_inpt = String::new();
                std::io::stdin().read_line(&mut buffer_inpt).ok();
                add_todo(&mut db, buffer_inpt);
            }
            "delete" => {
                print!("\x1B[2J\x1B[1;1H");
                let db_list = db.list();
                table::build_table(&db_list);
                println!("Which Task do you want to delete: ");
                std::io::stdin()
                    .read_line(&mut bufferDelete)
                    .expect("Didn't expected the buffer");
                let str_to_int: i32 = bufferDelete
                    .trim()
                    .parse()
                    .expect("Cannot convert this variable");
                db.delete(str_to_int as usize);
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
