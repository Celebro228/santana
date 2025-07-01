#[derive(Clone)]
pub struct Lang {
    pub _name: String,
    pub full_name: String,
    pub screen_language: String,
    pub screen_wifi_select: String,
    pub screen_wifi_password: String,
    pub screen_disk_select: String,
    pub screen_part_select: String,
    pub screen_part_select_for_efi: String,
    pub screen_usersetup: String,
    pub screen_usersetup_name: String,
    pub screen_usersetup_pass: String,
}

impl Lang {
    pub fn en() -> Self {
        Self {
            _name: "en".to_string(),
            full_name: "English".to_string(),
            screen_language: "Select language".to_string(),
            screen_wifi_select: "Select network".to_string(),
            screen_wifi_password: "Enter password".to_string(),
            screen_disk_select: "Select disk".to_string(),
            screen_part_select: "Select part".to_string(),
            screen_part_select_for_efi: "Select part for efi".to_string(),
            screen_usersetup: "Create user".to_string(),
            screen_usersetup_name: "Name".to_string(),
            screen_usersetup_pass: "Password".to_string(),
        }
    }

    pub fn ru() -> Self {
        Self {
            _name: "ru".to_string(),
            full_name: "Русский".to_string(),
            screen_language: "Выбор языка".to_string(),
            screen_wifi_select: "Выбор сети".to_string(),
            screen_wifi_password: "Введите пароль".to_string(),
            screen_disk_select: "Выбор диска".to_string(),
            screen_part_select: "Выбор раздела".to_string(),
            screen_part_select_for_efi: "Выбор раздела загрузчика".to_string(),
            screen_usersetup: "Создание пользователя".to_string(),
            screen_usersetup_name: "Имя".to_string(),
            screen_usersetup_pass: "Пароль".to_string(),
        }
    }
}


pub fn get_langs() -> Vec<Lang> {
    vec![Lang::en(), Lang::ru()]
}