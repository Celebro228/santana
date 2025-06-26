use crate::lang::{get_langs, Lang};


pub enum Screen {
    LanguageSelection,
    WifiSelection,
    DiskSelection,
    Partitioning,
    UserSetup,
    Installing,
    Complete,
}

pub enum Editing {
    Name,
    Password,
}

pub struct Data {
    pub name: String,
    pub password: String,
}

impl Data {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            password: String::new(),
        }
    }
}


pub struct App {
    pub user: Data,
    pub wifi: Data,
    pub languages: Vec<Lang>,
    pub language: Lang,
    pub screen: Screen,
    pub editing: Option<Editing>,
    pub select_num: usize,
    pub debug_mode: bool,
    pub logs: Vec<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            user: Data::new(),
            wifi: Data::new(),
            languages: get_langs(),
            language: Lang::en(),
            screen: Screen::LanguageSelection,
            editing: None,
            select_num: 0,
            debug_mode: false,
            logs: Vec::new(),
        }
    }


}