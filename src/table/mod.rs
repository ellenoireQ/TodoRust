use comfy_table::{Cell, Row, Table, presets::UTF8_FULL};

pub struct TableData {
    pub id: i32,
    pub task: String,
    pub state: bool,
}
pub fn build_table(tabs: &[TableData]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
        .set_header(vec!["ID", "Task", "State"]);

    for t in tabs {
        table.add_row(Row::from(vec![
            Cell::new(t.id),
            Cell::new(&t.task),
            Cell::new(t.state),
        ]));
    }
    println!("{table}");
}
