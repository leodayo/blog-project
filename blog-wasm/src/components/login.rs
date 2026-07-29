use crate::state::use_state;
use crate::storage;
use crate::{api, error::ApiError};
use leptos::prelude::*;
use leptos::{
    IntoView, component,
    reactive::{actions::Action, signal::RwSignal, traits::Set},
    view,
};

#[component]
pub fn LoginForm() -> impl IntoView {
    let state = use_state();
    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let error = RwSignal::new(None::<ApiError>);

    let login_action =
        Action::new_local(move |(input_username, input_password): &(String, String)| {
            let input_username = input_username.clone();
            let input_password = input_password.clone();
            let state = state.clone();

            async move {
                match api::login(&input_username, &input_password).await {
                    Ok(auth_resp) => {
                        state.token.set(Some(auth_resp.token.clone()));
                        state.user.set(Some(auth_resp.user));
                        storage::save_token(&auth_resp.token);
                        error.set(None);
                    }
                    Err(e) => error.set(Some(e)),
                }
            }
        });

    let loading = login_action.pending();

    view! {
        <div style="margin-bottom: 1rem;">
            <h3>"Login"</h3>
            <form on:submit=move |ev| {
                ev.prevent_default();
                error.set(None);
                login_action.dispatch((username.get(), password.get()));
            }>
                <div style="margin-bottom: 0.5rem;">
                    <input
                        type="text"
                        placeholder="Username"
                        prop:value=move || username.get()
                        on:input=move |ev| username.set(event_target_value(&ev))
                    />
                </div>
                <div style="margin-bottom: 0.5rem;">
                    <input
                        type="password"
                        placeholder="Password"
                        prop:value=move || password.get()
                        on:input=move |ev| password.set(event_target_value(&ev))
                    />
                </div>
                <button type="submit" disabled=move || loading.get()>
                    {move || if loading.get() { "Logging in..." } else { "Login" }}
                </button>

                {move || {
                    error.with(|err_opt| {
                        err_opt.as_ref().map(|err| view! {
                            <p style="color: red; margin-top: 0.5rem;">{format!("{}", err)}</p>
                        })
                    })
                }}
            </form>
        </div>
    }
}
