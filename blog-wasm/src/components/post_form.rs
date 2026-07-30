use crate::api;
use crate::error::ApiError;
use crate::state::use_state;
use leptos::prelude::*;

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
                }
            }
        });

    let loading = create_action.pending();

    view! {
        <div style="
            background: rgba(255, 255, 255, 0.6);
            backdrop-filter: blur(5px);
            border: 1px solid rgba(226, 232, 240, 0.8);
            border-radius: 8px;
            padding: 24px;
        ">
            <h4 style="margin: 0 0 16px 0; font-size: 1rem; font-weight: 500; color: #718096;">"Write Post"</h4>
            <form on:submit=move |ev| {
                ev.prevent_default();
                error.set(None);
                create_action.dispatch((title.get(), content.get()));
            }>
                <div style="margin-bottom: 12px;">
                    <input
                        type="text"
                        placeholder="Post Title"
                        style=crate::components::styles::INPUT_STYLE
                        onfocus="this.style.borderColor='#00f0ff'"
                        onblur="this.style.borderColor='rgba(226, 232, 240, 1)'"
                        prop:value=move || title.get()
                        on:input=move |ev| title.set(event_target_value(&ev))
                    />
                </div>
                <div style="margin-bottom: 16px;">
                    <textarea
                        placeholder="Share your data..."
                        style=format!("{} resize: vertical;", crate::components::styles::INPUT_STYLE)
                        onfocus="this.style.borderColor='#00f0ff'"
                        onblur="this.style.borderColor='rgba(226, 232, 240, 1)'"
                        prop:value=move || content.get()
                        on:input=move |ev| content.set(event_target_value(&ev))
                        rows=3
                    />
                </div>
                <div style="display: flex; justify-content: flex-end;">
                    <button
                        type="submit"
                        disabled=move || loading.get()
                        style=crate::components::styles::BTN_CYAN
                        onmouseenter="this.style.background='rgba(0, 240, 255, 0.05)';"
                        onmouseleave="this.style.background='transparent';"
                    >
                        {move || if loading.get() { "Publishing..." } else { "Submit" }}
                    </button>
                </div>

                {move || {
                    error.with(|err_opt| {
                        err_opt.as_ref().map(|err| view! {
                            <p style="color: red; font-size: 0.85rem; margin-top: 10px;">{format!("{}", err)}</p>
                        })
                    })
                }}
            </form>
        </div>
    }
}

#[component]
pub fn EditPostForm(
    id: i64,
    title: RwSignal<String>,
    content: RwSignal<String>,
    on_cancel: impl Fn(()) + Clone + 'static,
    on_saved: impl Fn(()) + Clone + 'static,
) -> impl IntoView {
    let state = use_state();
    let error = RwSignal::new(None::<ApiError>);

    let update_action =
        Action::new_local(move |(input_title, input_content): &(String, String)| {
            let input_title = input_title.clone();
            let input_content = input_content.clone();
            let post_id = id;
            let on_saved = on_saved.clone();

            async move {
                if let Some(token) = state.token.get() {
                    match api::update_post(post_id, &input_title, &input_content, &token).await {
                        Ok(updated) => {
                            state.posts.update(|posts| {
                                if let Some(existing) = posts.iter_mut().find(|p| p.id == id) {
                                    *existing = updated;
                                }
                            });
                            error.set(None);
                            on_saved(());
                        }
                        Err(e) => error.set(Some(e)),
                    }
                }
            }
        });

    let loading = update_action.pending();

    view! {
        <div style="padding: 10px 0;">
            <form on:submit=move |ev| {
                ev.prevent_default();
                error.set(None);
                update_action.dispatch((title.get(), content.get()));
            }>
                <div style="margin-bottom: 12px;">
                    <input
                        type="text"
                        style=crate::components::styles::INPUT_STYLE
                        prop:value=move || title.get()
                        on:input=move |ev| title.set(event_target_value(&ev))
                    />
                </div>
                <div style="margin-bottom: 12px;">
                    <textarea
                        style=crate::components::styles::INPUT_STYLE
                        prop:value=move || content.get()
                        on:input=move |ev| content.set(event_target_value(&ev))
                        rows=3
                    />
                </div>
                <div style="display: flex; gap: 8px; justify-content: flex-end;">
                    <button type="button" on:click=move |_| on_cancel(()) style="
                        background: transparent; border: 1px solid #cbd5e1; padding: 6px 16px; border-radius: 4px; cursor: pointer; font-size: 0.85rem;
                    ">"Cancel"</button>
                    <button type="submit" disabled=move || loading.get() style="
                        background: #00f0ff; color: white; border: none; padding: 6px 16px; border-radius: 4px; font-weight: 600; cursor: pointer; font-size: 0.85rem;
                    ">"Save"</button>
                </div>
            </form>
        </div>
    }
}
