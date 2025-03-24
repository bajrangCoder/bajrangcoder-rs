use crate::config::PERSONAL_DATA;
use dioxus::prelude::*;

const LEFT_CURLY_BRACE: &str = "{";
const RIGHT_CURLY_BRACE: &str = "}";

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            div { class: "container",
                div { class: "footer-content",
                    div { class: "footer-logo",
                        span { class: "logo-bracket", "{LEFT_CURLY_BRACE}" }
                        " {PERSONAL_DATA.name} "
                        span { class: "logo-bracket", "{RIGHT_CURLY_BRACE}" }
                    }
                    p { "© 2025 {PERSONAL_DATA.name}. All rights reserved." }
                }
            }
        }
    }
}
