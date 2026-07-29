use crate::components::post_form::EditPostForm;
use crate::dto::Post;
use crate::state::use_state;
use leptos::prelude::*;

#[component]
pub fn PostItem(post: Post, delete_action: Action<i64, ()>) -> impl IntoView {
    let state = use_state();
    let editing = RwSignal::new(false);

    let author_id = post.author_id;
    let post_id = post.id;
    let title_sig = RwSignal::new(post.title);
    let content_sig = RwSignal::new(post.content.unwrap_or_default());

    let formatted_date = post.created_at.format("%Y-%m-%d %H:%M:%S").to_string();

    let author_name = move || {
        state
            .user
            .get()
            .map(|current_user| {
                if current_user.id == author_id {
                    current_user.username
                } else {
                    format!("User #{}", author_id)
                }
            })
            .unwrap_or_else(|| format!("User #{}", author_id))
    };

    let is_author = move || {
        state
            .user
            .get()
            .map(|current_user| current_user.id == author_id)
            .unwrap_or(false)
    };

    let on_cancel = move |_| editing.set(false);
    let on_saved = move |_| editing.set(false);

    view! {
        <li class="post-animate-in" style="
            background: white;
            border-left: 3px solid #00c2e0;
            border-radius: 0 8px 8px 0;
            padding: 24px;
            box-shadow: 0 2px 12px rgba(0, 0, 0, 0.01);
            word-break: break-word;
            overflow-wrap: break-word;
            display: flex;
            flex-direction: column;
            height: auto;
        ">
            <Show
                when=move || editing.get()
                fallback=move || {
                    view! {
                        <div style="display: flex; flex-direction: column; height: auto; width: 100%;">
                            <h4 style="margin: 0 0 10px 0; font-size: 1.25rem; font-weight: 500; color: #1a202c; word-break: break-word;">
                                {move || title_sig.get()}
                            </h4>
                            <p style="margin: 0 0 20px 0; color: #4a5568; line-height: 1.6; font-size: 0.95rem; word-break: break-word; white-space: pre-wrap;">
                                {move || content_sig.get()}
                            </p>

                            <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 10px; margin-top: auto;">
                                <small style="color: #a0aec0; font-size: 0.8rem;">
                                    "Author: " {author_name} " @ " {formatted_date.clone()}
                                </small>

                                <Show when=is_author>
                                    <div style="display: flex; gap: 12px;">
                                        <button
                                            on:click=move |_| editing.set(true)
                                            style="background: transparent; color: #00c2e0; border: 1px solid rgba(0, 194, 224, 0.35); padding: 6px 14px; border-radius: 4px; font-size: 0.85rem; font-weight: 500; cursor: pointer;"
                                            onmouseenter="this.style.background='rgba(0, 194, 224, 0.04)';"
                                            onmouseleave="this.style.background='transparent';"
                                        >
                                            "Edit"
                                        </button>
                                        <button
                                            on:click=move |_| { delete_action.dispatch(post_id); }
                                            style="background: transparent; color: #ff7bf2; border: 1px solid rgba(255, 123, 242, 0.3); padding: 6px 14px; border-radius: 4px; font-size: 0.85rem; font-weight: 500; cursor: pointer;"
                                            onmouseenter="this.style.background='rgba(255, 123, 242, 0.05)';"
                                            onmouseleave="this.style.background='transparent';"
                                        >
                                            "Delete"
                                        </button>
                                    </div>
                                </Show>
                            </div>
                        </div>
                    }
                }
            >
                <EditPostForm
                    id=post_id
                    title=title_sig
                    content=content_sig
                    on_cancel=on_cancel.clone()
                    on_saved=on_saved.clone()
                />
            </Show>
        </li>
    }
}
