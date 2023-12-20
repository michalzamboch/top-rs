#![allow(dead_code)]

use fast_str::FastStr;
use ratatui::{prelude::*, widgets::*};

use crate::{
    types::traits::app_accessor::IAppAccessor,
    ui::{config, paths::*, util::*},
};

pub fn get_disk_table(app_handler: &dyn IAppAccessor) -> Table<'_> {
    let disks = app_handler.get_ui().get_table_handler(DISKS_TABLE_ID);
    get_disk_table_from_vec(&disks.get_data())
}

fn get_disk_table_from_vec(data: &[Vec<FastStr>]) -> Table<'static> {
    let rows = get_disk_rows(data);
    let constraints = get_constraints();
    let header = get_disks_header();
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);

    Table::new(rows, constraints)
        .header(header)
        .highlight_style(selected_style)
}

fn get_constraints() -> [Constraint; 4] {
    [
        Constraint::Max(get_terminal_width() as u16),
        Constraint::Min(config::PRETTY_BYTES_COL_LEN),
        Constraint::Min(config::PRETTY_BYTES_COL_LEN),
        Constraint::Min(config::PRETTY_BYTES_COL_LEN),
    ]
}
fn get_disk_rows(data: &[Vec<FastStr>]) -> impl Iterator<Item = Row<'static>> + '_ {
    data.iter().map(|item| {
        let cells = item.iter().map(|c| Cell::from(c.to_string()));
        Row::new(cells)
    })
}

fn get_disks_header() -> Row<'static> {
    let normal_style = Style::default().bg(Color::LightRed);
    let header_cells = ["Label", "Free", "Used", "Total"]
        .iter()
        .map(|h| Cell::from(*h));

    Row::new(header_cells).style(normal_style).height(1)
}
