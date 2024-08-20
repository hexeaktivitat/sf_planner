use anyhow::Error;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

use std::process::ExitCode;

use sf_planner::*;

fn main() -> Result<ExitCode, Error> {
    // init terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture);

    // set up crossterm
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = SfPlanner::new_from_json("data/items.json".into(), "data/recipes.json".into());
    let res = run_app(&mut terminal, &mut app);

    // gracefully exit application
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // handle errors

    Ok(ExitCode::SUCCESS)
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut SfPlanner) -> Result<(), Error> {
    loop {
        // render the UI to the terminal
        terminal.draw(|f| view(app, f))?;

        // keyboard input
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // disregard held inputs
                continue;
            }

            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    // shut down application
                    break;
                }
                KeyCode::Down => {
                    // select next in list
                }
                KeyCode::Up => {
                    // select previous in list
                }
                _ => {}
            }
        }
    }

    Ok(())
}
