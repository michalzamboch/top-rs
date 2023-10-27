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
use ratatui::{prelude::*, widgets::*};

use crate::backend::app::App;

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
    let tick_rate = Duration::from_millis(250);
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
            .unwrap_or_else(|| Duration::from_secs(1));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if KeyCode::Char('q') == key.code || KeyCode::Esc == key.code {
                    return Ok(());
                } else if KeyCode::Char('r') == key.code || KeyCode::F(5) == key.code {
                    app.on_tick();
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(80),
        ])
        .split(f.size());

    let cpu_gauge = get_cpu_gauge(app);
    f.render_widget(cpu_gauge, chunks[0]);
    
    let memory_gauge = get_memory_gauge(app);
    f.render_widget(memory_gauge, chunks[1]);
}

fn get_memory_gauge(app: &App) -> Gauge<'_> {
    Gauge::default()
        .block(Block::default().title("Memory usage").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Blue))
        .percent(app.get_memory_usage() as u16)
}

fn get_cpu_gauge(app: &App) -> Gauge<'_> {
    Gauge::default()
        .block(Block::default().title("CPU usage").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Green))
        .percent(app.get_total_cpu_usage() as u16)
}
