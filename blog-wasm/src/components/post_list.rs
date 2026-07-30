use crate::api;
use crate::components::post_item::PostItem;
use crate::state::use_state;
use leptos::prelude::*;

#[component]
pub fn PostList(loading: Memo<bool>) -> impl IntoView {
    let state = use_state();

    let delete_action = Action::new_local(move |id: &i64| {
        let id = *id;
        async move {
            if let Some(token) = state.token.get()
                && api::delete_post(id, &token).await.is_ok()
            {
                let old_posts = state.posts.get_untracked();
                let filtered = old_posts.into_iter().filter(|p| p.id != id).collect();
                state.posts.set(filtered);
                state.total.update(|t| *t = (*t - 1).max(0));
            }
        }
    });

    view! {
        <div style="margin-top: 20px;">
            <h3 style="
                font-size: 0.9rem;
                text-transform: uppercase;
                letter-spacing: 1.5px;
                color: #a0aec0;
                margin-bottom: 20px;
            ">"Feed / Total posts: " {move || state.total.get()}</h3>

            {move || {
                if loading.get() && state.posts.get().is_empty() {
                    view! {
                        <ul style="list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 16px;">
                            {vec![1, 2, 3].into_iter().map(|_| view! {
                                <li style="background: white; border-left: 3px solid #cbd5e1; border-radius: 0 8px 8px 0; padding: 24px; display: flex; flex-direction: column; gap: 12px;">
                                    <div class="skeleton-line" style="width: 40%; height: 20px;"></div>
                                    <div class="skeleton-line" style="width: 100%; height: 14px;"></div>
                                    <div class="skeleton-line" style="width: 85%; height: 14px;"></div>
                                    <div class="skeleton-line" style="width: 25%; height: 12px; margin-top: 8px;"></div>
                                </li>
                            }).collect::<Vec<_>>()}
                        </ul>
                    }.into_any()
                } else if state.posts.get().is_empty() {
                    view! {
                        <div style="text-align: center; padding: 40px; color: #a0aec0;">
                            "No posts found."
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <ul style="list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 16px;">
                            <For
                                each=move || state.posts.get()
                                key=|post| post.id
                                let:post
                            >
                                <PostItem post=post delete_action=delete_action />
                            </For>
                        </ul>
                    }.into_any()
                }
            }}
        </div>
    }
}
