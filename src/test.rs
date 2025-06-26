use std::process::Command;


fn main() {
    let output = Command::new("lspci")
        .arg("-mm")
        .output()
        .expect("failed to run lspci");

    let text = String::from_utf8_lossy(&output.stdout);

    let mut drivers = Drivers::default();
    println!("{:?}", drivers);

    for gpu in text.lines() {
        if gpu.contains("VGA") || gpu.contains("3D controller") {
            if gpu.to_lowercase().contains("nvidia") {
                drivers.nvidia = true;
            } else if gpu.to_lowercase().contains("amd") || gpu.to_lowercase().contains("ati") {
                drivers.amd = true;
            }
        }
    }


    println!("{:?}", drivers);
}


#[derive(Default, Debug)]
struct Drivers {
    nvidia: bool,
    amd: bool,
}

use ratatui::DefaultTerminal;
use crossterm::{event::{self, KeyCode}, terminal};

mod draw;
use draw::*;


#[derive(Debug, Clone)]
pub enum Screen {
    LanguageSelection,
    Welcome,
    WifiSelection,
    DiskSelection,
    Partitioning,
    ProgrammsSelection,
    UserSetup,
    Installing,
    Complete,
}

#[derive(Debug, Clone)]
pub struct App {
    pub language: String,
    pub current_step: Screen,
    pub selected_disk: String,
    pub user_name: String,
    pub user_password: String,
    pub install_progress: u16,
    pub log_messages: Vec<String>,
    pub debug_mode: bool,
}


fn main() {
    let terminal = ratatui::init();
    run(terminal);
    ratatui::restore();
}

fn run(mut terminal: DefaultTerminal) {

    let mut app = App {
        language: "en".to_string(),
        current_step: Screen::LanguageSelection,
        selected_disk: "".to_string(),
        user_name: "".to_string(),
        user_password: "".to_string(),
        install_progress: 0,
        log_messages: Vec::new(),
        debug_mode: false,
    };

    loop {
        terminal.draw(|frame| draw(frame, &mut app)).expect("Error draw procces");

        if let Some(key) = event::read().expect("Event error").as_key_press_event() {
            match key.code {
                KeyCode::Char('d') => app.debug_mode = !app.debug_mode,
                KeyCode::Enter => step_next(&mut app.current_step),
                KeyCode::Char('q') => step_back(&mut app.current_step),
                KeyCode::Esc => break,
                _ => {}
            }
        }
    }
}


fn step_back(step: &mut Screen) {
    *step = match step {
        Screen::LanguageSelection => Screen::LanguageSelection,
        Screen::Welcome => Screen::LanguageSelection,
        Screen::WifiSelection => Screen::Welcome,
        Screen::DiskSelection => Screen::WifiSelection,
        Screen::Partitioning => Screen::DiskSelection,
        Screen::ProgrammsSelection => Screen::Partitioning,
        Screen::UserSetup => Screen::ProgrammsSelection,
        Screen::Installing => Screen::UserSetup,
        Screen::Complete => Screen::Installing,
    }
}

fn step_next(step: &mut Screen) {
    *step = match step {
        Screen::LanguageSelection => Screen::Welcome,
        Screen::Welcome => Screen::WifiSelection,
        Screen::WifiSelection => Screen::DiskSelection,
        Screen::DiskSelection => Screen::Partitioning,
        Screen::Partitioning => Screen::ProgrammsSelection,
        Screen::ProgrammsSelection => Screen::UserSetup,
        Screen::UserSetup => Screen::Installing,
        Screen::Installing => Screen::Complete,
        Screen::Complete => Screen::Complete,
    }
}

use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Gauge, List, ListItem, Paragraph, Wrap},
    Frame,
};

#[derive(Debug, Clone)]
pub enum Screen {
    Welcome,
    DiskSelection,
    Partitioning,
    UserSetup,
    Installing,
    Complete,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub current_step: Screen,
    pub selected_disk: Option<String>,
    pub username: String,
    pub install_progress: u16,
    pub log_messages: Vec<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_step: Screen::Welcome,
            selected_disk: None,
            username: String::new(),
            install_progress: 0,
            log_messages: vec!["Добро пожаловать в Santana!".to_string()],
        }
    }
}

pub fn draw_ui(f: &mut Frame, app: &AppState) {
    let size = f.area();
    
    // Основной layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),    // Заголовок
            Constraint::Min(0),       // Основное содержимое
            Constraint::Length(3),    // Статус бар
        ])
        .split(size);

    // Заголовок
    draw_header(f, chunks[0]);
    
    // Основное содержимое в зависимости от шага
    match app.current_step {
        Screen::Welcome => draw_welcome(f, chunks[1]),
        Screen::DiskSelection => draw_disk_selection(f, chunks[1], app),
        Screen::Partitioning => draw_partitioning(f, chunks[1], app),
        Screen::UserSetup => draw_user_setup(f, chunks[1], app),
        Screen::Installing => draw_installing(f, chunks[1], app),
        Screen::Complete => draw_complete(f, chunks[1]),
    }
    
    // Статус бар
    draw_status_bar(f, chunks[2], app);
}

fn draw_header(f: &mut Frame, area: Rect) {
    let title = Paragraph::new("🏔️  Santana - Установщик Arch Linux")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    
    f.render_widget(title, area);
}

fn draw_welcome(f: &mut Frame, area: Rect) {
    let welcome_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Добро пожаловать в Santana!",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        )),
        Line::from(""),
        Line::from("Santana поможет вам установить Arch Linux быстро и легко."),
        Line::from(""),
        Line::from("Перед началом убедитесь что:"),
        Line::from("• У вас есть стабильное интернет-соединение"),
        Line::from("• Важные данные сохранены"),
        Line::from("• Система загружена в UEFI режиме (рекомендуется)"),
        Line::from(""),
        Line::from(Span::styled(
            "Нажмите Enter для продолжения или Ctrl+C для выхода",
            Style::default().fg(Color::Green)
        )),
    ];
    
    let paragraph = Paragraph::new(welcome_text)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title("Добро пожаловать"));
    
    f.render_widget(paragraph, area);
}

fn draw_disk_selection(f: &mut Frame, area: Rect, app: &AppState) {
    let disks = vec![
        ListItem::new("/dev/sda - 500GB SSD"),
        ListItem::new("/dev/sdb - 1TB HDD"),
        ListItem::new("/dev/nvme0n1 - 256GB NVMe"),
    ];
    
    let list = List::new(disks)
        .block(Block::default().borders(Borders::ALL).title("Выберите диск для установки"))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
        .highlight_symbol(">> ");
    
    f.render_widget(list, area);
}

fn draw_partitioning(f: &mut Frame, area: Rect, app: &AppState) {
    let text = vec![
        Line::from("Создание разделов..."),
        Line::from(""),
        Line::from(format!("Выбранный диск: {}", 
            app.selected_disk.as_ref().unwrap_or(&"Не выбран".to_string()))),
        Line::from(""),
        Line::from("Будут созданы следующие разделы:"),
        Line::from("• /dev/sda1 - 512MB - EFI System Partition"),
        Line::from("• /dev/sda2 - 4GB - Swap"),
        Line::from("• /dev/sda3 - Остальное - Root (/)"),
    ];
    
    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Разметка диска"))
        .wrap(Wrap { trim: true });
    
    f.render_widget(paragraph, area);
}

fn draw_user_setup(f: &mut Frame, area: Rect, app: &AppState) {
    let text = vec![
        Line::from("Настройка пользователя"),
        Line::from(""),
        Line::from(format!("Имя пользователя: {}", app.username)),
        Line::from(""),
        Line::from("Введите имя пользователя (только латинские буквы):"),
    ];
    
    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Настройка пользователя"))
        .wrap(Wrap { trim: true });
    
    f.render_widget(paragraph, area);
}

fn draw_installing(f: &mut Frame, area: Rect, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),    // Прогресс бар
            Constraint::Min(0),       // Лог
        ])
        .split(area);
    
    // Прогресс бар
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Прогресс установки"))
        .gauge_style(Style::default().fg(Color::Green))
        .percent(app.install_progress)
        .label(format!("{}%", app.install_progress));
    
    f.render_widget(gauge, chunks[0]);
    
    // Лог установки
    let log_items: Vec<ListItem> = app.log_messages
        .iter()
        .map(|msg| ListItem::new(msg.as_str()))
        .collect();
    
    let log_list = List::new(log_items)
        .block(Block::default().borders(Borders::ALL).title("Лог установки"));
    
    f.render_widget(log_list, chunks[1]);
}

fn draw_complete(f: &mut Frame, area: Rect) {
    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "🎉 Установка завершена успешно! 🎉",
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
        )),
        Line::from(""),
        Line::from("Arch Linux установлен и готов к использованию."),
        Line::from(""),
        Line::from("Рекомендации:"),
        Line::from("• Перезагрузите систему"),
        Line::from("• Обновите систему: sudo pacman -Syu"),
        Line::from("• Установите необходимые пакеты"),
        Line::from(""),
        Line::from(Span::styled(
            "Нажмите любую клавишу для выхода",
            Style::default().fg(Color::Yellow)
        )),
    ];
    
    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Установка завершена"))
        .wrap(Wrap { trim: true });
    
    f.render_widget(paragraph, area);
}

fn draw_status_bar(f: &mut Frame, area: Rect, app: &AppState) {
    let status_text = match app.current_step {
        Screen::Welcome => "Добро пожаловать | Enter - Продолжить, Ctrl+C - Выход",
        Screen::DiskSelection => "Выбор диска | ↑↓ - Навигация, Enter - Выбрать",
        Screen::Partitioning => "Разметка диска | Enter - Продолжить",
        Screen::UserSetup => "Настройка пользователя | Введите данные",
        Screen::Installing => "Установка | Пожалуйста, ждите...",
        Screen::Complete => "Завершено | Нажмите любую клавишу",
    };
    
    let status = Paragraph::new(status_text)
        .style(Style::default().fg(Color::White).bg(Color::Blue))
        .alignment(Alignment::Center)
        .block(Block::default());
    
    f.render_widget(status, area);
}