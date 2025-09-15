#![allow(non_snake_case)]

// Import Dioxus and necessary hooks/components
use dioxus::prelude::*;

// Imports for clipboard functionality
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::JsFuture;

mod text_removal;

const CONTAINER_CSS: Asset = asset!("assets/main.css");
const NAVBAR_CSS: Asset = asset!("assets/navbar.css");

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(Navbar)]
        #[route("/hide")]
        Hide,
        #[route("/seek")]
        Seek, 
        // #[route("/")]
        // #[redirect("/", || Route::Hide {} )]
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

fn main() {
    // Launch the web application
    launch(App);
}

// Define the main App component
#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CONTAINER_CSS }
        document::Stylesheet { href: NAVBAR_CSS }
        Router::<Route> {}
    }
}

#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            class: "navbar-split",
            div { class: "navbar-container",
                Link {
                    to: Route::Hide,
                    active_class: "active",
                    "Hide"
                }
                Link {
                    to: Route::Seek,
                    active_class: "active",
                    "Seek"
                }
            }
            div { class: "page-container",
                Outlet::<Route> {}
            }
        }
    }
}

#[component]
fn Hide() -> Element {
    let mut visible_text = use_signal(|| String::from("Hello, World!"));
    let mut hidden_text = use_signal(|| String::from("Hidden text"));
    let mut copy_button_text = use_signal(|| "Copy".to_string());

    let visible = visible_text.cloned();
    let hidden = hidden_text.cloned();
    let output_text = text_removal::create_secret(&visible, &hidden);

    rsx! {
        div { class: "widget-container",
            div { class: "input-group",
                label { "Visible Text Input" }
                input {
                    r#type: "text",
                    placeholder: "Enter some text here...",
                    oninput: move |event| visible_text.set(event.value())
                }
            }
            div { class: "input-group",
                label { "Hidden Text Input" }
                input {
                    r#type: "text",
                    placeholder: "Enter secret text here...",
                    oninput: move |event| hidden_text.set(event.value())
                }
            }
            div { class: "pre-wrapper",
                pre { "{output_text}" }
                button {
                    onclick: move |_| {
                        spawn({
                            let to_copy = output_text.clone();
                            let mut button_text = copy_button_text;
                            async move {
                                if let Some(window) = web_sys::window() {
                                    let clipboard = window.navigator().clipboard();
                                    let promise = clipboard.write_text(&to_copy);
                                    if JsFuture::from(promise).await.is_ok() {
                                        button_text.set("Copied!".to_string());
                                        TimeoutFuture::new(2000).await;
                                        button_text.set("Copy".to_string());
                                    }
                                }
                            }
                        });
                    },
                    "{copy_button_text}"
                }
            }
        }
    }
}

#[component]
fn Seek() -> Element {
    let mut combined_text = use_signal(|| String::new());
    let mut hidden_text = use_signal(|| String::new());

    let combined = combined_text.cloned();
    let hidden = text_removal::extract_secret(&combined);

    match hidden {
        Some(secret) => hidden_text.set(secret),
        None => hidden_text.set("No hidden text found.".to_string()),
    }

    rsx! {
        div { class: "widget-container",
            div { class: "input-group",
                label { "Combined Text Input" }
                input {
                    r#type: "text",
                    placeholder: "Enter combined text here...",
                    oninput: move |event| combined_text.set(event.value())
                }
            }
            div { class: "output-container",
                label { "Hidden Text Output" }
                pre { "{hidden_text}" }
            }
        }
    }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    let navigator = use_navigator();
    use_effect(move || {
        navigator.replace(Route::Hide);
    });

    // This component renders nothing, as it's only job is to trigger the redirect.
    rsx! {}
}
