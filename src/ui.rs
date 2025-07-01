use ratatui::{
    layout::{Constraint, Layout},
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

        Screen::DiskSelection => {
            let mut list_items = Vec::<ListItem>::new();
            for item in &app.disk_list {
                list_items.push(ListItem::new(item.0.clone()));
            }

            let mut state = ListState::default().with_selected(Some(app.disk));
            let list = List::new(list_items)
                .block(block.title(app.language.screen_disk_select.clone()))
                .white()
                .highlight_style(Style::new().bold().reversed())
                .highlight_symbol("> ")
                .repeat_highlight_symbol(true);

            frame.render_stateful_widget(list, main, &mut state);
        }
        Screen::Partitioning => {
            let part_list = app.disk_list
                .get(app.disk)
                .expect("Error to part list")
                .1.clone();

            let mut state = ListState::default().with_selected(Some(app.select_num));
            let list = List::new(part_list)
                .block(block.title(app.language.screen_part_select.clone()))
                .white()
                .highlight_style(Style::new().bold().reversed())
                .highlight_symbol("> ")
                .repeat_highlight_symbol(true);

            frame.render_stateful_widget(list, main, &mut state);
        }
        Screen::Efipart => {
            let part_list = app.disk_list
                .get(app.disk)
                .expect("Error to part list for efi")
                .1.clone();

            let mut state = ListState::default().with_selected(Some(app.select_num));
            let list = List::new(part_list)
                .block(block.title(app.language.screen_part_select_for_efi.clone()))
                .white()
                .highlight_style(Style::new().bold().reversed())
                .highlight_symbol("> ")
                .repeat_highlight_symbol(true);

            frame.render_stateful_widget(list, main, &mut state);
        }

        Screen::UserSetup => {
            frame.render_widget(block.title(app.language.screen_usersetup.clone()), main);

            let word_block = Block::bordered();

            let [_, word_layout, _] = Layout::vertical([
                Constraint::Fill(1),
                Constraint::Length(6),
                Constraint::Fill(1),
            ]).areas(main);
            let [_, word_layout, _] = Layout::horizontal([
                Constraint::Fill(1),
                Constraint::Length(50),
                Constraint::Fill(1),
            ]).areas(word_layout);
            let [name_layout, pass_layout] = Layout::vertical([
                Constraint::Length(3),
                Constraint::Length(3),
            ]).areas(word_layout);


            let mut pass_word = String::new();
            for _ in 0..app.user.password.len() {
                pass_word.push('*');
            }

            let name = Paragraph::new(app.user.name.clone() + 
                if let Editing::Name = app.editing.clone()
                .expect("Editing name error") {"█"} else {""})
                .block(word_block.clone().title(app.language.screen_usersetup_name.clone()));

            let pass = Paragraph::new(pass_word + 
                if let Editing::Password = app.editing.clone()
                .expect("Editing password error") {"█"} else {""})
                .block(word_block.clone().title(app.language.screen_usersetup_pass.clone()));

            frame.render_widget(name, name_layout);
            frame.render_widget(pass, pass_layout);
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