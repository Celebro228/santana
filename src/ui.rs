use ratatui::{
    layout::{Constraint, Layout, Direction, Rect},
    widgets::{Block, Paragraph, List, ListItem, ListState, Padding}, 
    style::{Style, Stylize}, 
    Frame,
};
use crate::app::*;


pub fn ui(frame: &mut Frame, app: &App) {
    let horizonal = Layout::horizontal([Constraint::Max(50), Constraint::Min(50)]);
    let [photo, main] = horizonal.areas(frame.area());

    let block = Block::bordered().padding(Padding::proportional(2));


    match app.screen {
        Screen::LanguageSelection => {
            let mut list_items = Vec::<ListItem>::new();
            for item in &app.language_list {
                list_items.push(ListItem::new(item.full_name.clone()));
            }

            let mut state = ListState::default().with_selected(Some(app.select_num));
            let list = List::new(list_items)
                .block(block.title(app.language.screen_language.clone()))
                .white()
                .highlight_style(Style::new().bold().reversed())
                .highlight_symbol("> ")
                .repeat_highlight_symbol(true);

            frame.render_stateful_widget(list, main, &mut state);
        }

        Screen::WifiSelection => {
            let mut state = ListState::default().with_selected(Some(app.select_num));
            let list = List::new(app.wifi_list.clone())
                .block(block.title(app.language.screen_wifi_select.clone()))
                .white()
                .highlight_style(Style::new().bold().reversed())
                .highlight_symbol("> ")
                .repeat_highlight_symbol(true);

            frame.render_stateful_widget(list, main, &mut state);

            if let Some(_) = app.editing {
                let popup_block = Block::bordered().title(app.language.screen_wifi_password.clone());

                let [_, popup_layout, _] = Layout::vertical([
                    Constraint::Fill(1),
                    Constraint::Length(3),
                    Constraint::Fill(1),
                ]).areas(frame.area());

                let [_, popup_layout, _] = Layout::horizontal([
                    Constraint::Fill(1),
                    Constraint::Length(50),
                    Constraint::Fill(1),
                ]).areas(popup_layout);

                let text = Paragraph::new(app.wifi.password.clone() + "█")
                    .block(popup_block);
                    
                frame.render_widget(text, popup_layout);
            }
        }
        _ => {}
    }


    if app.debug_mode && app.logs.len() != 0 {
        let mut state = ListState::default().with_selected(Some(app.logs.len() - 1));
        let list = List::new(app.logs.clone())
            .white()
            .highlight_style(Style::new().bold().reversed());

        frame.render_stateful_widget(list, photo, &mut state);
    }
}