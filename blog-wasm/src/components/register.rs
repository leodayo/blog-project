use crate::{api, state::use_state, storage};
use leptos::prelude::*;

#[component]
pub fn RegisterForm() -> impl IntoView {
    let state = use_state();
    let username = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let error_message = RwSignal::new(None::<String>);

    let register_action = Action::new_local(
        move |(username, email, password): &(String, String, String)| {
            let username = username.clone();
            let email = email.clone();
            let password = password.clone();
            let state = state.clone();

            async move {
                match api::register(&username, &email, &password).await {
                    Ok(auth_resp) => {
                        state.token.set(Some(auth_resp.token.clone()));
                        state.user.set(Some(auth_resp.user.clone()));
                        storage::save_token(&auth_resp.token);
                        storage::save_user(&auth_resp.user);
                        error_message.set(None);
                    }
                    Err(e) => match e {
                        crate::error::ApiError::Http { status, .. } => {
                            error_message
                                .set(Some(format!("Registration failed. Status: {}", status)));
                        }
                        _ => error_message.set(Some("Network connection error".to_string())),
                    },
                }
            }
        },
    );

    let loading = register_action.pending();

    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            error_message.set(None);
            register_action.dispatch((username.get(), email.get(), password.get()));
        }>
            <div style="display: flex; flex-direction: column; gap: 10px; margin-bottom: 12px;">
                <input
                    type="text"
                    placeholder="Username"
                    style=crate::pages::home::INPUT_STYLE
                    onfocus="this.style.borderColor='#00f0ff'"
                    onblur="this.style.borderColor='#e2e8f0'"
                    prop:value=move || username.get()
                    on:input=move |ev| username.set(event_target_value(&ev))
                />
                <input
                    type="email"
                    placeholder="Email Address"
                    style=crate::pages::home::INPUT_STYLE
                    onfocus="this.style.borderColor='#00f0ff'"
                    onblur="this.style.borderColor='#e2e8f0'"
                    prop:value=move || email.get()
                    on:input=move |ev| email.set(event_target_value(&ev))
                />
                <input
                    type="password"
                    placeholder="Password"
                    style=crate::pages::home::INPUT_STYLE
                    onfocus="this.style.borderColor='#00f0ff'"
                    onblur="this.style.borderColor='#e2e8f0'"
                    prop:value=move || password.get()
                    on:input=move |ev| password.set(event_target_value(&ev))
                />
            </div>

            <div style="display: flex; justify-content: flex-end; align-items: center;">
                <button type="submit" disabled=move || loading.get() style=crate::pages::home::BTN_CYAN>
                    {move || if loading.get() { "Creating..." } else { "Register" }}
                </button>
            </div>

            {move || {
                error_message.get().map(|msg| view! {
                    <p style="color: #e53e3e; font-size: 0.85rem; margin-top: 8px; font-weight: 500;">{msg}</p>
                })
            }}
        </form>
    }
}
