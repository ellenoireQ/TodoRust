pub mod database;
mod function;
pub mod table;
use std::{
    process::Command,
    thread::{self},
    time::{self},
};

use crate::{database::Database, function::add_todo};

fn clear() {
    let output = Command::new("clear")
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Command stdout:\n{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed with stderr:\n{}", stderr);
    }
}
fn main() {
    // inferred at database
    let mut db = database::DatabaseContext::new();

    let mut state = true;
    while state {
        clear();

        let table_datas = db.list();
        table::build_table(table_datas);
        let mut buffer = String::new();

        let mut bufferDelete = String::new();
        println!("Please input Command add | mark | delete | exit");
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Didn't expected the answer");

        let input = buffer.trim();

        match input {
            /*          *
             *  Add     *
             *          */
            "add" => {
                clear();
                let table_datas = db.list();
                table::build_table(&table_datas);
                println!("Input your task: (e.g, Cooking with mom tonight)");
                let mut buffer_inpt = String::new();
                std::io::stdin().read_line(&mut buffer_inpt).ok();

                let dbs = Database {
                    todos: buffer_inpt,
                    state: false,
                };

                add_todo(&mut db, dbs);
            }
            "mark" => {
                clear();
                let table_datas = db.list();
                table::build_table(&table_datas);
                println!("Which task do you want to checklist");
                let mut buffer_delete = String::new();

                std::io::stdin()
                    .read_line(&mut buffer_delete)
                    .expect("Expected string but you inputted another variable");
                match buffer_delete.trim().parse::<usize>() {
                    Ok(index) => {
                        if index < table_datas.len() {
                            db.mark(index);
                        } else {
                            //
                        }
                    }
                    Err(_) => {}
                }
            }
            "delete" => {
                clear();
                let db_list = db.list();
                table::build_table(&db_list);
                println!("Which Task do you want to delete: (e.g, 0)");
                std::io::stdin()
                    .read_line(&mut bufferDelete)
                    .expect("Didn't expected the buffer");

                match bufferDelete.trim().parse::<usize>() {
                    Ok(index) => {
                        if index < db_list.len() {
                            db.delete(index);
                        } else {
                            println!("index: {index} not found, back to main menu");
                            let ten_millis = time::Duration::from_millis(2000);
                            thread::sleep(ten_millis);
                        }
                    }
                    Err(_) => {
                        println!("Invalidc input. Please enter a valid task number.");
                    }
                }
            }
            "exit" => {
                clear();
                println!("\n==============================");
                println!(" Licensed under CC0 1.0 Universal.");
                println!(" https://creativecommons.org/publicdomain/zero/1.0/");
                println!("==============================\n");
                state = false;
            }
            _ => {
                clear();
                println!("You didnt inputted program, GETTT OUTTTT!!!");

                println!("\n==============================");
                println!(" Licensed under CC0 1.0 Universal.");
                println!(" https://creativecommons.org/publicdomain/zero/1.0/");
                println!("==============================\n");

                state = false;
            }
        }
    }
    let table_datas = &db.list();
    table::build_table(table_datas);
}
