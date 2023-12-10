#![allow(dead_code)]

use ratatui::{prelude::*, widgets::*};

use crate::{
    types::traits::app_accessor::IAppAccessor,
    ui::{config, paths::*},
};

pub fn get_disk_table(app_handler: &dyn IAppAccessor) -> Table<'_> {
    let disks = app_handler.get_ui().get_table_handler(DISKS_TABLE_ID);
    get_disk_table_from_vec(&disks.get_box())
}

fn get_disk_table_from_vec(data: &[Vec<String>]) -> Table<'static> {
    let rows = get_disk_rows(data);
    let header = get_disks_header();
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);

    Table::new(rows)
        .header(header)
        .highlight_style(selected_style)
        .widths(&[
            Constraint::Max(config::NAME_COL_MAX_LEN),
            Constraint::Min(config::PRETTY_BYTES_COL_LEN),
            Constraint::Min(config::PRETTY_BYTES_COL_LEN),
            Constraint::Min(config::PRETTY_BYTES_COL_LEN),
        ])
}

fn get_disk_rows(data: &[Vec<String>]) -> impl Iterator<Item = Row<'static>> + '_ {
    data.iter().map(|item| {
        let cells = item.iter().map(|c| Cell::from(c.clone()));
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
