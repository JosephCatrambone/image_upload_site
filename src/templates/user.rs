use perseus::prelude::*;
use perseus::plugins::Plugins;
use serde::{Serialize, Deserialize};
use sycamore::prelude::*;
use uuid::Uuid;

use crate::data_models::user;


// Note: #[auto_scope] instead of #[perseus::template_rx] for v0.4.
#[auto_scope]
// EXCERPT_START
fn index_page<G: Html>(cx: Scope, state: &UserStateRx) -> View<G> {
    view! { cx,
        h1 { (format!(
            "Hello, {}!",
            state.name.get()
        )) }
        input(
            placeholder = "Name",
            bind:value = state.name
        )
        a(href = "about") { "About" }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Welcome to Perseus!" }
    }
}

pub fn get_templates<G: Html>() -> Vec<Template<G>> {
    // .view for stateless, .view_with_unreactive_state, and .view_with_reactive_state.
    vec![
        //Template::build("index").view(index_page).head(head).build(),
        Template::build("user")
            .view_with_state(index_page)
            .build_state_fn(get_build_state)
            .build(),
    ]
}

