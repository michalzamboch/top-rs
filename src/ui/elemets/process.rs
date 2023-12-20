use fast_str::FastStr;
use ratatui::{prelude::*, widgets::*};

use crate::{
    types::traits::app_accessor::IAppAccessor,
    ui::{config, paths::*, util::get_terminal_width},
};

pub fn get_process_table(app_handler: &dyn IAppAccessor) -> Table<'_> {
    let table = app_handler.get_ui().get_table_handler(PROCESSES_TABLE_ID);
    get_process_table_from_vec(&table.get_data())
}

fn get_process_table_from_vec(data: &[Vec<FastStr>]) -> Table<'static> {
    let rows = get_process_rows(data);
    let widths = get_constraints();
    let header = get_process_header();
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);

    Table::new(rows, widths)
        .header(header)
        .highlight_style(selected_style)
}

fn get_constraints() -> [Constraint; 6] {
    [
        Constraint::Min(config::PID_COL_LEN),
        Constraint::Max(get_terminal_width() as u16),
        Constraint::Min(config::CPU_USAGE_COL_LEN),
        Constraint::Min(config::PRETTY_BYTES_COL_LEN),
        Constraint::Min(config::PRETTY_BYTES_COL_LEN),
        Constraint::Min(config::PRETTY_BYTES_COL_LEN),
    ]
}

fn get_process_rows(data: &[Vec<FastStr>]) -> impl Iterator<Item = Row<'static>> + '_ {
    data.iter().map(|item| {
        let cells = item.iter().map(|c| Cell::from(c.to_string()));
        Row::new(cells)
    })
}

fn get_process_header() -> Row<'static> {
    let normal_style = Style::default().bg(config::PROCESS_COLOR);

    let header_cells = [
        "Pid [P]",
        "Process [N]",
        "Cpu [C]",
        "Memory [M]",
        "Read [R]",
        "Write [W]",
    ]
    .iter()
    .map(|h| Cell::from(*h));

    Row::new(header_cells).style(normal_style).height(1)
}
