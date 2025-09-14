#![allow(non_snake_case)]

// Import Dioxus and necessary hooks/components
use dioxus::prelude::*;

// Imports for clipboard functionality
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::JsFuture;

const CSS: Asset = asset!("assets/main.css");

fn main() {
    // Launch the web application
    launch(App);
}

// Define the main App component
fn App() -> Element {
    // State for the visible text input
    let mut visible_text = use_signal(|| String::new());
    // State for the hidden text input
    let mut hidden_text = use_signal(|| String::new());
    // State for the copy button's label to provide user feedback
    let mut copy_button_text = use_signal(|| "Copy".to_string());

    // A derived string that combines the two inputs for the output area
    let output_text = format!(
        "Visible Text: \"{}\"\nHidden Text: \"{}\"",
        visible_text(),
        hidden_text()
    );

    rsx! {
        // Simple styling for layout and appearance
        document::Stylesheet { href: CSS }
        // Main application container
        div { class: "container",
            h1 { "Dioxus Input and Copyable Output" }

            // Group for the first text input
            div { class: "input-group",
                label { "Visible Text Input" }
                input {
                    r#type: "text",
                    placeholder: "Enter some text here...",
                    // Update the `visible_text` signal on every input event
                    oninput: move |event| visible_text.set(event.value())
                }
            }

            // Group for the "hidden" (password) text input
            div { class: "input-group",
                label { "Hidden Text Input" }
                input {
                    r#type: "text",
                    placeholder: "Enter secret text here...",
                    // Update the `hidden_text` signal on every input event
                    oninput: move |event| hidden_text.set(event.value())
                }
            }

            // The output area which is read-only
            div { class: "pre-wrapper",
                // `pre` tag is used to preserve formatting, like newlines
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
