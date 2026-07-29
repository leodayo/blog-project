use crate::pages::home::HomePage;
use crate::state::provide_state;
use leptos::IntoView;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_state();

    view! {
        <Router>
            <main style="min-height: 100vh; display: flex; flex-direction: column;">
                <Routes fallback=|| "Page not found.">
                    <Route path=path!("/*") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}
