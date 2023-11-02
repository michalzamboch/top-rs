use std::{
    error::Error,
    io::{self, Stdout},
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

use crate::{
    backend::app::App,
    types::{app_trait::IApp, sort_by::SortBy},
};

use super::{app_handler::AppHandler, config, ui_builder::*};

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
            .unwrap_or_else(|| Duration::from_secs(config::REFRESH_MILIS));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                let exit = handle_input(key, &mut app_handler.app);
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

fn handle_input(key: KeyEvent, app: &mut App) -> bool {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => return true,
        KeyCode::F(5) => app.on_tick(),
        KeyCode::Char('c') => app.sort_processes_by(SortBy::Cpu),
        KeyCode::Char('C') => app.sort_processes_by(SortBy::CpuReverse),
        KeyCode::Char('p') => app.sort_processes_by(SortBy::Pid),
        KeyCode::Char('P') => app.sort_processes_by(SortBy::PidReverse),
        KeyCode::Char('n') => app.sort_processes_by(SortBy::Name),
        KeyCode::Char('N') => app.sort_processes_by(SortBy::NameReverse),
        KeyCode::Char('m') => app.sort_processes_by(SortBy::Memory),
        KeyCode::Char('M') => app.sort_processes_by(SortBy::MemoryReverse),
        KeyCode::Char('r') => app.sort_processes_by(SortBy::DiskRead),
        KeyCode::Char('R') => app.sort_processes_by(SortBy::DiskReadReverse),
        KeyCode::Char('w') => app.sort_processes_by(SortBy::DiskWrite),
        KeyCode::Char('W') => app.sort_processes_by(SortBy::DiskWriteReverse),
        _ => (),
    }

    false
}
