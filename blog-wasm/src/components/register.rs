use crate::{api, error::ApiError, state::use_state, storage};
use leptos::prelude::*;
use leptos::{
    IntoView, component,
    reactive::{actions::Action, signal::RwSignal, traits::Set},
    view,
};

#[component]
pub fn RegisterForm() -> impl IntoView {
    let state = use_state();
    let username = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let error = RwSignal::new(None::<ApiError>);

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
                        state.user.set(Some(auth_resp.user));
                        storage::save_token(&auth_resp.token);
                        error.set(None);
                    }
                    Err(e) => error.set(Some(e)),
                }
            }
        },
    );

    let loading = register_action.pending();

    view! {
        <div style="margin-bottom: 1rem;">
            <h3>"Register"</h3>
            <form on:submit=move |ev| {
                ev.prevent_default();
                error.set(None);
                register_action.dispatch((username.get(), email.get(), password.get()));
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
                        type="email"
                        placeholder="Email"
                        prop:value=move || email.get()
                        on:input=move |ev| email.set(event_target_value(&ev))
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
                    {move || if loading.get() { "Registering..." } else { "Register" }}
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
