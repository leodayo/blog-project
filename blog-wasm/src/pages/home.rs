use crate::api;
use crate::components::login::LoginForm;
use crate::components::post_form::CreatePostForm;
use crate::components::post_list::PostList;
use crate::components::register::RegisterForm;
use crate::state::use_state;
use crate::storage;
use leptos::prelude::*;

const DEFAULT_LIMIT: i64 = 10;
const DEFAULT_OFFSET: i64 = 0;

#[component]
pub fn HomePage() -> impl IntoView {
    let state = use_state();
    let is_authenticated = move || state.token.get().is_some();

    let state_for_action = state.clone();
    let load_posts = Action::new_local(move |_| {
        let state = state_for_action.clone();
        async move {
            if let Ok(resp) = api::list_posts(DEFAULT_LIMIT, DEFAULT_OFFSET).await {
                state.posts.set(resp.posts);
                state.total.set(resp.total);
            }
        }
    });

    Effect::new(move |_| {
        load_posts.dispatch(());
    });

    view! {
        <div style="max-width: 900px; margin: 0 auto; padding: 20px; font-family: system-ui, sans-serif;">
            <h1 style="color: #00bcd4; border-bottom: 2px solid #00bcd4; padding-bottom: 0.5rem;">"Blog"</h1>
            <div style="display: flex; gap: 30px; flex-wrap: wrap;">
                <div style="flex: 1; min-width: 250px;">
                    <Show
                        when=move || !is_authenticated()
                        fallback=move || {
                            view! {
                                <div style="background: #f0f0f0; padding: 1rem; border-radius: 4px;">
                                    <p><strong>"Welcome, "</strong> {move || state.user.get().map(|u| u.username).unwrap_or_default()}</p>
                                    <button
                                        on:click=move |_| {
                                            state.token.set(None);
                                            state.user.set(None);
                                            storage::remove_token();
                                        }
                                        style="background: #d32f2f; color: white; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer;"
                                    >
                                        "Logout"
                                    </button>
                                </div>
                            }.into_any()
                        }
                    >
                        <LoginForm />
                        <hr style="margin: 1rem 0;" />
                        <RegisterForm />
                    </Show>
                </div>
                <div style="flex: 2; min-width: 300px;">
                    <Show when=move || is_authenticated()>
                        <CreatePostForm on_post_created=move |_| { load_posts.dispatch(()); } />
                    </Show>
                    <PostList />
                </div>
            </div>
        </div>
    }
}
