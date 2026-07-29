use crate::{
    dto::{Post, User},
    storage,
};
use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct AppState {
    pub token: RwSignal<Option<String>>,
    pub user: RwSignal<Option<User>>,
    pub posts: RwSignal<Vec<Post>>,
    pub total: RwSignal<i64>,
}

pub fn provide_state() {
    let token = RwSignal::new(storage::load_token());
    let user = RwSignal::new(storage::load_user());
    let posts = RwSignal::new(Vec::new());
    let total = RwSignal::new(i64::default());

    let state = AppState {
        token,
        user,
        posts,
        total,
    };

    provide_context(state);
}

pub fn use_state() -> AppState {
    expect_context::<AppState>()
}
