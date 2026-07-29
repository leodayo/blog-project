use crate::pages::home::HomePage;
use crate::state::provide_state;
use leptos::IntoView;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::Route;
use leptos_router::components::Router;
use leptos_router::components::Routes;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_state();

    view! {
        <Router>
            <main>
                <Routes fallback= || "Page not found.">
                    <Route path=path!("/") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}
