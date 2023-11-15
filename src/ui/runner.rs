#![allow(dead_code)]

use std::{
    error::Error,
    io::{self, Stdout},
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, *},
    execute,
    terminal::*,
};
use ratatui::prelude::*;

use crate::types::{enums::sort_by::SortBy, traits::app_accessor::IAppAccessor};

use super::{app_handler::AppHandler, config, paths::*, ui_builder::*};

pub fn start() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = create_app(&mut terminal);
    restore_terminal(&mut terminal)?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn create_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    let tick_rate = Duration::from_millis(config::REFRESH_MILIS);
    let app_handler = AppHandler::new();
    run_app(terminal, app_handler, tick_rate)?;

    Ok(())
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app_handler: AppHandler,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| handle_ui(f, &app_handler))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_millis(config::REFRESH_MILIS));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                let exit = handle_input(key, &mut app_handler);
                if exit {
                    return Ok(());
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app_handler.update();
            last_tick = Instant::now();
        }
    }
}

fn resize_network_spark_lines(app_handler: &AppHandler) {
    let receive_line = app_handler.get_ui().get_spar_line(RECEIVED_SPARK_LINE_ID);
    receive_line.resize();
    let trasmitted_line = app_handler.get_ui().get_spar_line(TRASMITTED_SPARK_LINE_ID);
    trasmitted_line.resize();
}

fn handle_input(key: KeyEvent, app_handler: &mut AppHandler) -> bool {
    if key.kind != KeyEventKind::Press {
        return false;
    }

    match key.code {
        KeyCode::Esc => return true,

        KeyCode::F(5) => app_handler.update(),
        KeyCode::F(6) => app_handler.hard_update(),
        KeyCode::F(7) => app_handler.pause_unpause(),

        KeyCode::Char('c') => app_handler.sort_processes_by(SortBy::Cpu),
        KeyCode::Char('C') => app_handler.sort_processes_by(SortBy::CpuReverse),
        KeyCode::Char('p') => app_handler.sort_processes_by(SortBy::Pid),
        KeyCode::Char('P') => app_handler.sort_processes_by(SortBy::PidReverse),
        KeyCode::Char('n') => app_handler.sort_processes_by(SortBy::Name),
        KeyCode::Char('N') => app_handler.sort_processes_by(SortBy::NameReverse),
        KeyCode::Char('m') => app_handler.sort_processes_by(SortBy::Memory),
        KeyCode::Char('M') => app_handler.sort_processes_by(SortBy::MemoryReverse),
        KeyCode::Char('r') => app_handler.sort_processes_by(SortBy::DiskRead),
        KeyCode::Char('R') => app_handler.sort_processes_by(SortBy::DiskReadReverse),
        KeyCode::Char('w') => app_handler.sort_processes_by(SortBy::DiskWrite),
        KeyCode::Char('W') => app_handler.sort_processes_by(SortBy::DiskWriteReverse),

        KeyCode::Down => app_handler.process_down(),
        KeyCode::Up => app_handler.process_up(),
        KeyCode::Home => app_handler.first_process(),
        KeyCode::End => app_handler.last_process(),
        _ => (),
    }

    false
}
