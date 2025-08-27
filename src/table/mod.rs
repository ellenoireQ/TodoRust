use comfy_table::{Cell, Row, Table, presets::UTF8_FULL};

use crate::database::{Database, DatabaseContext};

pub struct TableData {
    pub id: i32,
    pub task: String,
    pub state: bool,
}
pub fn build_table(task: &Vec<Database>) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
        .set_header(vec!["Index", "Task", "state"]);

    for (index, items) in task.iter().enumerate() {
        table.add_row(Row::from(vec![
            Cell::new(index),
            Cell::new(&items.todos),
            Cell::new(&items.state),
        ]));
    }

    println!("{table}");
}
