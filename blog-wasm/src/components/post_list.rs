use crate::components::post_item::PostItem;
use crate::state::use_state;
use leptos::prelude::*;

#[component]
pub fn PostList() -> impl IntoView {
    let state = use_state();

    view! {
        <div>
            <h3>"Posts"</h3>
            {move || {
                if state.posts.get().is_empty() {
                    view! { <p>"No posts yet."</p> }.into_any()
                } else {
                    view! {
                        <ul style="list-style: none; padding: 0;">
                            <For
                                each=move || state.posts.get()
                                key=|post| post.id
                                let:post
                            >
                                <PostItem post=post />
                            </For>
                        </ul>
                    }.into_any()
                }
            }}
        </div>
    }
}
