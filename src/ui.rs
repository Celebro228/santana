use ratatui::{
    layout::{Constraint,Layout},
    widgets::{Block, Paragraph, List, ListItem, ListState, Padding}, 
    style::{Style, Stylize}, 
    Frame,
};
use crate::{app::*, lang::*};


pub fn ui(frame: &mut Frame, app: &App) {
    let horizonal = Layout::horizontal([Constraint::Max(50), Constraint::Min(50)]);
    let [photo, main] = horizonal.areas(frame.area());

    let block = Block::bordered().padding(Padding::proportional(2));


    match app.screen {
        Screen::LanguageSelection => {
            let mut list_items = Vec::<ListItem>::new();
            for item in &app.languages {
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
        _ => {}
    }

    frame.render_widget(Paragraph::new("Okay").centered(), photo);
}
