#[derive(Clone)]
pub struct Lang {
    pub name: String,
    pub full_name: String,
    pub screen_language: String,
    pub screen_wifi_select: String,
    pub screen_wifi_password: String,
}

impl Lang {
    pub fn en() -> Self {
        Self {
            name: "en".to_string(),
            full_name: "English".to_string(),
            screen_language: "Select language".to_string(),
            screen_wifi_select: "Select network".to_string(),
            screen_wifi_password: "Enter password".to_string(),
        }
    }

    pub fn ru() -> Self {
        Self {
            name: "ru".to_string(),
            full_name: "Русский".to_string(),
            screen_language: "Выбор языка".to_string(),
            screen_wifi_select: "Выбор сети".to_string(),
            screen_wifi_password: "Введите пароль".to_string(),
        }
    }
}


pub fn get_langs() -> Vec<Lang> {
    vec![Lang::en(), Lang::ru()]
}