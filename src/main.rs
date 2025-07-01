use ratatui::DefaultTerminal;
use crossterm::event::{self, KeyCode};

mod lang;

mod app;
use app::*;

mod ui;
use ui::ui;


/*
Задачи:
Управление дисками:
Выбор диска
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
                        if app.wifi_check() {
                            app.screen = Screen::DiskSelection;
                            app.editing = None;
                            app.set_disk_list();
                            app.disk = 0;
                        } else {
                            app.screen = Screen::WifiSelection;
                            app.set_wifi_list();
                        }
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
                            app.wifi_connect();
                            if app.wifi_check() {
                                app.screen = Screen::DiskSelection;
                                app.editing = None;
                                app.time_sync();
                                app.set_disk_list();
                                app.disk = 0;
                            }
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
                    KeyCode::Esc => {
                        app.screen = Screen::WifiSelection;
                        app.select_num = 0;
                    }
                    KeyCode::Up => app.disk = (app.disk - 1).max(0),
                    KeyCode::Down => app.disk = (app.disk + 1).min(app.disk_list.len() - 1),
                    KeyCode::Enter => {
                        app.screen = Screen::Partitioning;
                        app.select_num = 0;
                    }
                    _ => {}
                }
                Screen::Partitioning => match key.code {
                    KeyCode::Esc => {
                        app.screen = Screen::DiskSelection;
                        app.disk = 0;
                    }
                    KeyCode::Up => app.select_num = (app.select_num - 1).max(0),
                    KeyCode::Down => app.select_num = (app.select_num + 1).min(app.disk_list
                        .get(app.disk)
                        .expect("Error to part list")
                        .1.len() - 1),
                    KeyCode::Enter => {
                        app.disk_tom = app.disk_list
                            .get(app.disk)
                            .expect("Error to part list")
                            .1.get(app.select_num)
                            .expect("Error to part")
                            .clone();
                        app.logs.push("Select disk tom: ".to_string() + &app.disk_tom);

                        if app.efi_check() {
                            app.screen = Screen::Efipart;
                            app.select_num = 0;
                        } else {
                            app.screen = Screen::UserSetup;
                        }
                    }
                    _ => {}
                }
                Screen::Efipart => match key.code {
                    KeyCode::Esc => {
                        app.screen = Screen::Partitioning;
                        app.select_num = 0;
                    }
                    KeyCode::Up => app.select_num = (app.select_num - 1).max(0),
                    KeyCode::Down => app.select_num = (app.select_num + 1).min(app.disk_list
                        .get(app.disk)
                        .expect("Error to part list for efi")
                        .1.len() - 1),
                    KeyCode::Enter => {
                        app.disk_tom_efi = Some(app.disk_list
                            .get(app.disk)
                            .expect("Error to part list for efi")
                            .1.get(app.select_num)
                            .expect("Error to part for efi")
                            .clone());
                        app.logs.push("Select disk tom for efi: ".to_string() + &app.disk_tom_efi.clone().expect("Error disk efi name"));

                        app.screen = Screen::UserSetup;
                        app.user.name.clear();
                        app.user.password.clear();
                        app.editing = Some(Editing::Name);
                    }
                    _ => {}
                }

                Screen::UserSetup => match key.code {
                    KeyCode::Esc => {
                        app.screen = Screen::Partitioning;
                        app.select_num = 0;
                    }
                    KeyCode::Up => app.editing = Some(Editing::Name),
                    KeyCode::Down => app.editing = Some(Editing::Password),
                    KeyCode::Enter => match app.editing.clone().expect("Editing is None") {
                        Editing::Name => app.editing = Some(Editing::Password),
                        Editing::Password => {
                            app.logs.push("User create: ".to_string() + &app.user.name);
                            app.screen = Screen::Installing;
                        }
                    }
                    KeyCode::Char(value) => match app.editing.clone().expect("Editing is None") {
                        Editing::Name => app.user.name.push(value),
                        Editing::Password => app.user.password.push(value),
                    }
                    KeyCode::Backspace => match app.editing.clone().expect("Editing is None") {
                        Editing::Name => { app.user.name.pop(); }
                        Editing::Password => { app.user.password.pop(); }
                    }
                    _ => {}
                }
                _ => {}
            }
        }
    }
}