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

    let show_auth_panel = RwSignal::new(false);
    let auth_mode_register = RwSignal::new(false);

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

    let reload_trigger = move || {
        load_posts.dispatch(());
    };

    load_posts.dispatch(());

    view! {
        <div style="max-width: 750px; width: 100%; margin: 0 auto; padding: 40px 20px;">
            <header style="
                display: flex;
                justify-content: space-between;
                align-items: center;
                margin-bottom: 40px;
                padding-bottom: 15px;
                border-bottom: 1px solid rgba(0, 194, 224, 0.22);
            ">
                <h1 style="
                    margin: 0;
                    font-size: 1.7rem;
                    font-weight: 500;
                    letter-spacing: -0.5px;
                    background: linear-gradient(120deg, #00c2e0 35%, #ff7bf2 65%);
                    -webkit-background-clip: text;
                    -webkit-text-fill-color: transparent;
                    background-clip: text;
                ">
                    "blog."
                </h1>

                <div style="display: flex; align-items: center; gap: 16px;">
                    <Show
                        when=is_authenticated
                        fallback=move || {
                            view! {
                                <button
                                    on:click=move |_| show_auth_panel.update(|v| *v = !*v)
                                    style="background: transparent; color: #00c2e0; border: 1px solid rgba(0, 194, 224, 0.5); padding: 6px 14px; border-radius: 4px; font-size: 0.85rem; cursor: pointer; font-weight: 500;"
                                    onmouseenter="this.style.background='rgba(0, 194, 224, 0.04)';"
                                    onmouseleave="this.style.background='transparent';"
                                >
                                    {move || if show_auth_panel.get() { "Close" } else { "Sign In" }}
                                </button>
                            }.into_any()
                        }
                    >
                        <span style="font-size: 0.85rem; color: #718096; font-weight: 500;">
                            {move || state.user.get().map(|u| u.username).unwrap_or_default()}
                        </span>
                        <button
                            on:click=move |_| {
                                state.token.set(None);
                                state.user.set(None);
                                storage::remove_token();
                                show_auth_panel.set(false);
                            }
                            style="background: transparent; color: #ff7bf2; border: 1px solid rgba(255, 123, 242, 0.4); padding: 6px 14px; border-radius: 4px; font-size: 0.85rem; cursor: pointer;"
                            onmouseenter="this.style.background='rgba(255, 123, 242, 0.05)';"
                            onmouseleave="this.style.background='transparent';"
                        >
                            "Logout"
                        </button>
                    </Show>
                </div>
            </header>

            <Show when=move || show_auth_panel.get() && !is_authenticated()>
                <div style=format!("{} margin-bottom: 30px;", crate::components::styles::CARD_STYLE)>
                    <div style="display: flex; gap: 15px; margin-bottom: 20px; border-bottom: 1px solid #e2e8f0; padding-bottom: 10px;">
                        <span
                            on:click=move |_| auth_mode_register.set(false)
                            style=move || format!("cursor: pointer; font-size: 0.9rem; font-weight: bold; color: {};", if !auth_mode_register.get() { "#00c2e0" } else { "#a0aec0" })
                        >
                            "Login"
                        </span>
                        <span
                            on:click=move |_| auth_mode_register.set(true)
                            style=move || format!("cursor: pointer; font-size: 0.9rem; font-weight: bold; color: {};", if auth_mode_register.get() { "#00c2e0" } else { "#a0aec0" })
                        >
                            "Register"
                        </span>
                    </div>

                    <Show
                        when=move || auth_mode_register.get()
                        fallback=move || view! { <LoginForm /> }.into_any()
                    >
                        <RegisterForm />
                    </Show>
                </div>
            </Show>

            <div style="display: flex; flex-direction: column; gap: 35px;">
                <Show when=is_authenticated>
                    <CreatePostForm on_post_created=move |_| { reload_trigger(); } />
                </Show>

                <PostList loading=load_posts.pending() />
            </div>
        </div>
    }
}
