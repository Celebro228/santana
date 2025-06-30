use ratatui::DefaultTerminal;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

mod lang;

mod app;
use app::*;

mod ui;
use ui::ui;


/*
Задачи:
Выбор WiFi
Управление дисками:
Выбор тома
Удаление тома
Выбор пустого пространства
*/


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
                        app.language = app.language_list.get(app.select_num).unwrap().clone();
                        app.logs.push("Up".to_string());
                    }
                    KeyCode::Down => {
                        app.select_num = (app.select_num + 1).min(app.language_list.len() - 1);
                        app.language = app.language_list.get(app.select_num).unwrap().clone();
                        app.logs.push("Down".to_string());
                    }
                    KeyCode::Enter => {
                        app.screen = Screen::WifiSelection;
                        app.set_wifi_list();
                        app.select_num = 0;
                    }
                    _ => {}
                }

                Screen::WifiSelection => match key.code {
                    KeyCode::Esc => match app.editing {
                        None => {
                            app.screen = Screen::LanguageSelection;
                            app.language = app.language_list.get(0).unwrap().clone();
                            app.select_num = 0;
                        }
                        Some(_) => app.editing = None
                    }
                    KeyCode::Up => {
                        if let None = app.editing {
                            app.select_num = (app.select_num - 1).max(0);
                        }
                    }
                    KeyCode::Down => {
                        if let None = app.editing {
                            app.select_num = (app.select_num + 1).min(app.wifi_list.len() - 1);
                        }
                    }
                    KeyCode::Enter => match app.editing {
                        None => {
                            app.wifi.name = app.wifi_list.get(app.select_num)
                                .expect("failed to wifi list get network name")
                                .clone();
                            app.wifi.password.clear();
                            app.editing = Some(Editing::Password);

                            app.logs.push("Network select: ".to_string() + &app.wifi.name);
                        }
                        Some(_) => {
                            app.screen = Screen::DiskSelection;
                            app.editing = None;
                            app.wifi_connect();
                            app.select_num = 0;
                        }
                    }
                    KeyCode::Char(value) => if let Some(_) = app.editing {
                        app.wifi.password.push(value);
                    }
                    KeyCode::Backspace => if let Some(_) = app.editing {
                        app.wifi.password.pop();
                    }
                    _ => {}
                }

                Screen::DiskSelection => match key.code {
                    KeyCode::Esc => match app.editing {
                        None => {
                            app.screen = Screen::WifiSelection;
                            app.select_num = 0;
                        }
                        Some(_) => app.editing = None
                    }
                    _ => {}
                }
                _ => {}
            }
        }
    }
}