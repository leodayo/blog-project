use crate::api;
use crate::dto::Post;
use crate::error::ApiError;
use crate::state::use_state;
use leptos::prelude::*;
use leptos::{
    IntoView, component,
    reactive::{actions::Action, signal::RwSignal, traits::Set},
    view,
};

#[component]
pub fn CreatePostForm(on_post_created: impl Fn(()) + Clone + 'static) -> impl IntoView {
    let state = use_state();
    let title = RwSignal::new(String::new());
    let content = RwSignal::new(String::new());
    let error = RwSignal::new(None::<ApiError>);

    let create_action =
        Action::new_local(move |(input_title, input_content): &(String, String)| {
            let input_title = input_title.clone();
            let input_content = input_content.clone();
            let state = state.clone();
            let on_post_created = on_post_created.clone();

            async move {
                if let Some(token) = state.token.get() {
                    match api::create_post(&input_title, &input_content, &token).await {
                        Ok(new_post) => {
                            state.posts.update(|posts| posts.push(new_post));
                            title.set(String::new());
                            content.set(String::new());
                            error.set(None);
                            on_post_created(());
                        }
                        Err(e) => error.set(Some(e)),
                    }
                } else {
                    error.set(Some(ApiError::Http {
                        status: 401,
                        body: "Not authenticated".to_string(),
                    }));
                }
            }
        });

    let loading = create_action.pending();

    view! {
        <div style="margin: 1rem 0; padding: 1rem; border: 1px solid #ccc; border-radius: 4px;">
            <h4>"Create Post"</h4>
            <form on:submit=move |ev| {
                ev.prevent_default();
                error.set(None);
                create_action.dispatch((title.get(), content.get()));
            }>
                <div style="margin-bottom: 0.5rem;">
                    <input
                        type="text"
                        placeholder="Title"
                        prop:value=move || title.get()
                        on:input=move |ev| title.set(event_target_value(&ev))
                    />
                </div>
                <div style="margin-bottom: 0.5rem;">
                    <textarea
                        placeholder="Content"
                        prop:value=move || content.get()
                        on:input=move |ev| content.set(event_target_value(&ev))
                        rows=4
                        style="width: 100%;"
                    />
                </div>
                <button type="submit" disabled=move || loading.get()>
                    {move || if loading.get() { "Creating..." } else { "Create" }}
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

#[component]
pub fn EditPostForm(
    post: Post,
    on_cancel: impl Fn(()) + Clone + 'static,
    on_saved: impl Fn(()) + Clone + 'static,
) -> impl IntoView {
    let state = use_state();
    let title = RwSignal::new(post.title.clone());
    let content = RwSignal::new(post.content.clone().unwrap_or_default());
    let error = RwSignal::new(None::<ApiError>);

    let update_action =
        Action::new_local(move |(input_title, input_content): &(String, String)| {
            let input_title = input_title.clone();
            let input_content = input_content.clone();
            let post_id = post.id;
            let state = state.clone();
            let on_saved = on_saved.clone();

            async move {
                if let Some(token) = state.token.get() {
                    match api::update_post(post_id, &input_title, &input_content, &token).await {
                        Ok(updated) => {
                            state.posts.update(|posts| {
                                if let Some(existing) = posts.iter_mut().find(|p| p.id == post_id) {
                                    *existing = updated;
                                }
                            });
                            error.set(None);
                            on_saved(());
                        }
                        Err(e) => error.set(Some(e)),
                    }
                } else {
                    error.set(Some(ApiError::Http {
                        status: 401,
                        body: "Not authenticated".to_string(),
                    }));
                }
            }
        });

    let loading = update_action.pending();

    view! {
        <div>
            <h4>"Edit Post"</h4>
            <form on:submit=move |ev| {
                ev.prevent_default();
                error.set(None);
                update_action.dispatch((title.get(), content.get()));
            }>
                <div style="margin-bottom: 0.5rem;">
                    <input
                        type="text"
                        placeholder="Title"
                        prop:value=move || title.get()
                        on:input=move |ev| title.set(event_target_value(&ev))
                    />
                </div>
                <div style="margin-bottom: 0.5rem;">
                    <textarea
                        placeholder="Content"
                        prop:value=move || content.get()
                        on:input=move |ev| content.set(event_target_value(&ev))
                        rows=4
                        style="width: 100%;"
                    />
                </div>
                <button type="submit" disabled=move || loading.get()>
                    {move || if loading.get() { "Updating..." } else { "Update" }}
                </button>
                <button type="button" on:click=move |_| on_cancel(()) style="margin-left: 0.5rem;">"Cancel"</button>

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
