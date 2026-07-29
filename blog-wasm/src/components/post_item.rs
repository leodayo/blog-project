use crate::api;
use crate::components::post_form::EditPostForm;
use crate::dto::Post;
use crate::state::use_state;
use leptos::prelude::*;

#[component]
pub fn PostItem(post: Post) -> impl IntoView {
    let state = use_state();
    let editing = RwSignal::new(false);
    let post_inner = post.clone();

    let author_id = post.author_id;
    let is_author = move || state.user.get().map(|u| u.id == author_id).unwrap_or(false);

    let delete_action = Action::new_local(move |id: &i64| {
        let id = *id;
        let state = state.clone();
        async move {
            if let Some(token) = state.token.get() {
                if api::delete_post(id, &token).await.is_ok() {
                    state.posts.update(|posts| posts.retain(|p| p.id != id));
                }
            }
        }
    });

    let on_cancel = move |_| editing.set(false);
    let on_saved = move |_| editing.set(false);

    view! {
        <li style="border: 1px solid #ccc; padding: 1rem; margin-bottom: 0.5rem; border-radius: 4px;">
            <Show
                when=move || editing.get()
                fallback=move || {
                    let post_for_delete = post_inner.clone();
                    view! {
                        <div>
                            <h4 style="margin: 0 0 0.5rem 0;">{post_inner.title.clone()}</h4>
                            <p style="margin: 0 0 0.5rem 0;">{post_inner.content.clone().unwrap_or_default()}</p>
                            <small style="color: #666;">"By " {post_inner.author_id} " at " {post_inner.created_at.to_string()}</small>
                            <Show when=is_author>
                                <div style="margin-top: 0.5rem;">
                                    <button on:click=move |_| editing.set(true)>"Edit"</button>
                                    <button
                                        on:click=move |_| { delete_action.dispatch(post_for_delete.id); }
                                        style="margin-left: 0.5rem;"
                                    >
                                        "Delete"
                                    </button>
                                </div>
                            </Show>
                        </div>
                    }
                }
            >
                <EditPostForm
                    post=post.clone()
                    on_cancel=on_cancel.clone()
                    on_saved=on_saved.clone()
                />
            </Show>
        </li>
    }
}
