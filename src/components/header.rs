use crate::{
    config::PERSONAL_DATA,
    icons::{Github, Linkedin, Twitter},
};
use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

const LEFT_CURLY_BRACE: &str = "{";
const RIGHT_CURLY_BRACE: &str = "}";

#[component]
pub fn Header() -> Element {
    // mobile menu related stuffs
    let mut is_menu_open = use_signal(|| false);
    let mut menu_transform = use_signal(|| "rotateY(90deg)".to_string());
    let mut menu_display = use_signal(|| "none".to_string());
    let mut hamburger_spans = use_signal(|| {
        (
            "none".to_string(), // first span transform
            "1".to_string(),    // middle span opacity
            "none".to_string(), // last span transform
        )
    });

    let toggle_menu = move |_| {
        if !is_menu_open() {
            menu_display.set("block".to_string());
            menu_transform.set("rotateY(90deg)".to_string());

            hamburger_spans.set((
                "rotate(45deg) translate(5px, 5px)".to_string(),
                "0".to_string(),
                "rotate(-45deg) translate(7px, -6px)".to_string(),
            ));

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let body = document.body().unwrap();
            body.style().set_property("overflow", "hidden").unwrap();

            spawn(async move {
                TimeoutFuture::new(50).await;
                is_menu_open.set(true);
                menu_transform.set("rotateY(0deg)".to_string());
            });
        } else {
            menu_transform.set("rotateY(90deg)".to_string());

            // Reset hamburger
            hamburger_spans.set(("none".to_string(), "1".to_string(), "none".to_string()));

            spawn(async move {
                TimeoutFuture::new(300).await;
                is_menu_open.set(false);
                menu_display.set("none".to_string());

                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                let body = document.body().unwrap();
                body.style().set_property("overflow", "").unwrap();
            });
        }
    };

    rsx! {
        header {
            div { class: "container",
                div { class: "logo",
                    span { class: "logo-bracket", "{LEFT_CURLY_BRACE}" }
                    " {PERSONAL_DATA.name} "
                    span { class: "logo-bracket", "{RIGHT_CURLY_BRACE}" }
                }
                nav {
                    ul {
                        li {
                            a { href: "#home", "Home" }
                        }
                        li {
                            a { href: "#about", "About" }
                        }
                        li {
                            a { href: "#projects", "Projects" }
                        }
                        li {
                            a { href: "#contact", "Contact" }
                        }
                    }
                }
                button {
                    class: "menu-toggle",
                    aria_label: "Toggle menu",
                    onclick: toggle_menu,
                    span { style: "transform: {hamburger_spans().0}" }
                    span { style: "opacity: {hamburger_spans().1}" }
                    span { style: "transform: {hamburger_spans().2}" }
                }
            }
        }

        div {
            class: if is_menu_open() { "mobile-menu active" } else { "mobile-menu" },
            style: "display: {menu_display}; transform: {menu_transform};",
            nav { class: "mobile-menu-links",
                a { class: "mobile-menu-link active-link", href: "#home", "Home" }
                a { class: "mobile-menu-link", href: "#about", "About" }
                a { class: "mobile-menu-link", href: "#projects", "Projects" }
                a { class: "mobile-menu-link", href: "#contact", "Contact" }
            }
            div { class: "menu-social-links",
                a { aria_label: "GitHub", href: "{PERSONAL_DATA.github}", Github {} }
                a { aria_label: "LinkedIn", href: "{PERSONAL_DATA.linkedin}", Linkedin {} }
                a { aria_label: "Twitter", href: "{PERSONAL_DATA.twitter}", Twitter {} }
            }
        }

        div {
            class: if is_menu_open() { "menu-overlay active" } else { "menu-overlay" },
            onclick: toggle_menu,
        }
    }
}
