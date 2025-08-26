use comfy_table::{Cell, Row, Table, presets::UTF8_FULL};

pub fn build_table() {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
        .set_header(vec!["ID", "Task", "State"]);

    table.add_row(vec!["1", "Alice", "95"]);
    table.add_row(vec!["2", "Bob", "88"]);
    table.add_row(Row::from(vec![
        Cell::new("3"),
        Cell::new("Charlie"),
        Cell::new("100").fg(comfy_table::Color::Green),
    ]));

    println!("{table}");
}
