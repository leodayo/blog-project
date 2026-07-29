use web_sys::{Storage, window};

const TOKEN_KEY: &str = "blog_token";

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

pub fn remove_token() {
    if let Some(storage) = get_storage() {
        let _ = storage.remove_item(TOKEN_KEY);
    }
}
