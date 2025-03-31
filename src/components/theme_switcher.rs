use crate::{config::THEMES, icons::Palette};
use dioxus::prelude::*;
use web_sys::window;

#[component]
pub fn ThemeSwitcher() -> Element {
    let mut show_options = use_signal(|| false);
    let mut current_theme = use_signal(|| String::new());

    let toggle_options = move |_: Event<MouseData>| {
        show_options.toggle();
    };
    let theme_option_class = if show_options() { "active" } else { "" };

    use_effect(move || {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(theme)) = storage.get_item("theme") {
                    if let Some(document) = window.document() {
                        if let Some(element) = document.document_element() {
                            current_theme.set(theme.clone());
                            let _ = element.set_attribute("data-theme", &theme);
                        }
                    }
                }
            }
        }
    });

    let mut set_theme = move |theme_id: &str| {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("theme", theme_id);
            }
            if let Some(document) = window.document() {
                if let Some(element) = document.document_element() {
                    let _ = element.set_attribute("data-theme", theme_id);
                }
            }
        }
        current_theme.set(theme_id.to_string());
        show_options.set(false);
    };

    rsx! {
        div { class: "theme-switcher",
            button { class: "theme-toggle", onclick: toggle_options, Palette {} }
            div { class: "theme-options {theme_option_class}",
                {
                    THEMES
                        .iter()
                        .map(|theme| {
                            let active_theme_item_class = if current_theme() == theme.id.to_string()
                            {
                                "active"
                            } else {
                                ""
                            };
                            rsx! {
                                button {
                                    key: "{theme.id}",
                                    class: "theme-option {active_theme_item_class}",
                                    onclick: move |_| set_theme(theme.id),
                                    span { class: "color-preview {theme.preview_class}" }
                                    span { "{theme.name}" }
                                }
                            }
                        })
                }
            }
        }
    }
}
