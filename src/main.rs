use ratatui::DefaultTerminal;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

mod lang;
use lang::*;

mod app;
use app::*;

mod ui;
use ui::ui;


fn main() {
    let terminal = ratatui::init();
    run(terminal);
    ratatui::restore();
}

fn run(mut terminal: DefaultTerminal) {
    let mut app = App::new();

    loop {
        terminal.draw(|f| ui(f, &app)).expect("Error draw");

        if let Some(key) = event::read().expect("Error input").as_key_press_event() {
            match app.screen {
                Screen::LanguageSelection => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('d') => app.debug_mode = !app.debug_mode,
                    KeyCode::Up => {
                        app.select_num = (app.select_num - 1).max(0);
                        app.language = app.languages.get(app.select_num).unwrap().clone();
                    }
                    KeyCode::Down => {
                        app.select_num = (app.select_num + 1).min(app.languages.len() - 1);
                        app.language = app.languages.get(app.select_num).unwrap().clone();
                    }
                    KeyCode::Enter => {
                        app.screen = Screen::WifiSelection;
                        app.select_num = 0;
                    }
                    _ => {}
                }
                Screen::WifiSelection => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        app.screen = Screen::LanguageSelection;
                        app.language = app.languages.get(0).unwrap().clone();
                        app.select_num = 0;
                    }
                    KeyCode::Up => {
                        app.select_num = (app.select_num - 1).max(0);
                    }
                    KeyCode::Down => {
                        app.select_num = (app.select_num + 1).min(app.languages.len() - 1);
                    }
                    KeyCode::Enter => app.screen = Screen::WifiSelection,
                    _ => {}
                }
                _ => {}
            }
        }
    }
}