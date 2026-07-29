use crate::dto::User;
use web_sys::{Storage, window};

const TOKEN_KEY: &str = "blog_token";
const USER_KEY: &str = "blog_user";

fn get_storage() -> Option<Storage> {
    window()?.local_storage().ok()?
}

pub fn save_token(token: &str) {
    if let Some(storage) = get_storage() {
        let _ = storage.set_item(TOKEN_KEY, token);
    }
}

pub fn load_token() -> Option<String> {
    get_storage()?.get_item(TOKEN_KEY).ok()?
}

pub fn save_user(user: &User) {
    if let Some(storage) = get_storage() {
        if let Ok(serialized) = serde_json::to_string(user) {
            let _ = storage.set_item(USER_KEY, &serialized);
        }
    }
}

pub fn load_user() -> Option<User> {
    let storage = get_storage()?;
    let raw = storage.get_item(USER_KEY).ok()??;
    serde_json::from_str(&raw).ok()
}

pub fn remove_token() {
    if let Some(storage) = get_storage() {
        let _ = storage.remove_item(TOKEN_KEY);
        let _ = storage.remove_item(USER_KEY);
    }
}
