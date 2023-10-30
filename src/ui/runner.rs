use std::{
    error::Error,
    io::{self, Stdout},
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

use crate::{backend::app::App, types::sort_by::SortBy};

use super::{config, ui_builder::ui};

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
    let app = App::new();
    run_app(terminal, app, tick_rate)?;

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
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(config::REFRESH_MILIS));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if config::EXIT_KEY_CODES.contains(&key.code) {
                    return Ok(());
                } else if KeyCode::F(5) == key.code {
                    app.on_tick();
                } else if KeyCode::Char('c') == key.code {
                    app.sort_processes_by(SortBy::Cpu);
                } else if KeyCode::Char('C') == key.code {
                    app.sort_processes_by(SortBy::CpuReverse);
                } else if KeyCode::Char('p') == key.code {
                    app.sort_processes_by(SortBy::Pid);
                } else if KeyCode::Char('P') == key.code {
                    app.sort_processes_by(SortBy::PidReverse);
                } else if KeyCode::Char('n') == key.code {
                    app.sort_processes_by(SortBy::Name);
                } else if KeyCode::Char('N') == key.code {
                    app.sort_processes_by(SortBy::NameReverse);
                } else if KeyCode::Char('m') == key.code {
                    app.sort_processes_by(SortBy::Memory);
                } else if KeyCode::Char('M') == key.code {
                    app.sort_processes_by(SortBy::MemoryReverse);
                } else if KeyCode::Char('r') == key.code {
                    app.sort_processes_by(SortBy::DiskRead);
                } else if KeyCode::Char('R') == key.code {
                    app.sort_processes_by(SortBy::DiskReadReverse);
                } else if KeyCode::Char('w') == key.code {
                    app.sort_processes_by(SortBy::DiskWrite);
                } else if KeyCode::Char('W') == key.code {
                    app.sort_processes_by(SortBy::DiskWriteReverse);
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}
