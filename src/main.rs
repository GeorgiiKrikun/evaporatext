#![allow(non_snake_case)]

// Import Dioxus and necessary hooks/components
use dioxus::prelude::*;

// Imports for clipboard functionality
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::JsFuture;

mod text_removal;

const CONTAINER_CSS: Asset = asset!("assets/main.css");
const NAVBAR_CSS: Asset = asset!("assets/navbar.css");

fn main() {
    // Launch the web application
    launch(App);
}

#[derive(Clone, PartialEq)]
enum Page {
    Combine,
    Split,
}

// Define the main App component
fn App() -> Element {
    let mut current_page = use_signal(|| Page::Combine);
    let button_hide_class = match current_page.cloned() {
        Page::Combine => "active",
        Page::Split => "",
    };
    let button_reveal_class = match current_page.cloned() {
        Page::Combine => "",
        Page::Split => "active",
    };

    rsx! {
        document::Stylesheet { href: CONTAINER_CSS }
        document::Stylesheet { href: NAVBAR_CSS }
        div {
            class: "navbar-split",
            div {
                class: "navbar-container",
                button {
                    class: button_hide_class,
                    onclick: move |_| current_page.set(Page::Combine),
                    "Hide"
                }
                button {
                    class: button_reveal_class,
                    onclick: move |_| current_page.set(Page::Split),
                    "Reveal"
                }
            }
            div { class: "page-container",
                if current_page.cloned() == Page::Combine {
                    CombinePage {}
                } else {
                    SplitPage {}
                }
            }
        }
    }
}

fn CombinePage() -> Element {
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

fn SplitPage() -> Element {
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
