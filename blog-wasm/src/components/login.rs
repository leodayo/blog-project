use crate::api;
use crate::state::use_state;
use crate::storage;
use leptos::prelude::*;

#[component]
pub fn LoginForm() -> impl IntoView {
    let state = use_state();
    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let error_message = RwSignal::new(None::<String>);

    let login_action =
        Action::new_local(move |(input_username, input_password): &(String, String)| {
            let input_username = input_username.clone();
            let input_password = input_password.clone();
            let state = state.clone();

            async move {
                match api::login(&input_username, &input_password).await {
                    Ok(auth_resp) => {
                        state.token.set(Some(auth_resp.token.clone()));
                        state.user.set(Some(auth_resp.user.clone()));
                        storage::save_token(&auth_resp.token);
                        storage::save_user(&auth_resp.user);
                        error_message.set(None);
                    }
                    Err(e) => match e {
                        crate::error::ApiError::Http { status, .. } => {
                            if status == 401 || status == 400 {
                                error_message.set(Some("Invalid username or password".to_string()));
                            } else {
                                error_message.set(Some(format!("Server error: {}", status)));
                            }
                        }
                        _ => error_message.set(Some("Network connection error".to_string())),
                    },
                }
            }
        });

    let loading = login_action.pending();

    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            error_message.set(None);
            login_action.dispatch((username.get(), password.get()));
        }>
            <div style="display: flex; flex-wrap: wrap; gap: 10px; margin-bottom: 12px;">
                <div style="flex: 1; min-width: 180px;">
                    <input
                        type="text"
                        placeholder="Username"
                        style=crate::components::styles::INPUT_STYLE
                        onfocus="this.style.borderColor='#00f0ff'"
                        onblur="this.style.borderColor='#e2e8f0'"
                        prop:value=move || username.get()
                        on:input=move |ev| username.set(event_target_value(&ev))
                    />
                </div>
                <div style="flex: 1; min-width: 180px;">
                    <input
                        type="password"
                        placeholder="Password"
                        style=crate::components::styles::INPUT_STYLE
                        onfocus="this.style.borderColor='#00f0ff'"
                        onblur="this.style.borderColor='#e2e8f0'"
                        prop:value=move || password.get()
                        on:input=move |ev| password.set(event_target_value(&ev))
                    />
                </div>
            </div>

            <div style="display: flex; justify-content: flex-end; align-items: center;">
                <button type="submit" disabled=move || loading.get() style=crate::components::styles::BTN_CYAN>
                    {move || if loading.get() { "Connecting..." } else { "Login" }}
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
