//! An application that can receive and show notifications.

use dioxus::prelude::*;

/// Version info.
const VERSION_WITH_GIT: &str = env!("VERSION_WITH_GIT");
/// CSS for the app.
const CSS: Asset = asset!("/assets/app.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CSS }
        Hello {}
    }
}

#[component]
fn Hello() -> Element {
    rsx! {
        div { class: "absolute top-2 right-2 badge badge-sm badge-ghost", "{VERSION_WITH_GIT}" }

        div { class: "hero bg-base-300 min-h-screen",
            div { class: "hero-content text-center",
                div { class: "max-w-md",
                    h1 { class: "text-5xl font-bold", "Hello, world!" }
                    p { class: "py-6", "Welcome to a minimal Dioxus template using daisyUI." }
                    button { class: "btn btn-primary", "Get Started" }
                }
            }
        }
    }
}
