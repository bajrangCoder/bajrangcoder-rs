use crate::config::PERSONAL_DATA;
use dioxus::prelude::*;

const LEFT_CURLY_BRACE: &str = "{";
const RIGHT_CURLY_BRACE: &str = "}";

#[component]
pub fn Header() -> Element {
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
                button { class: "menu-toggle", aria_label: "Toggle menu",
                    span {}
                    span {}
                    span {}
                }
            }
        }

        div { class: "mobile-menu",
            div { class: "mobile-menu-content",
                div { class: "mobile-menu-header",
                    div { class: "logo",
                        a { href: "#home",
                            "DEV"
                            span { "FOLIO" }
                        }
                    }
                    button {
                        aria_label: "Close mobile menu",
                        id: "mobile-menu-close",
                        i { class: "fas fa-times" }
                    }
                }
                nav { class: "mobile-nav",
                    ul {
                        li {
                            a { class: "mobile-nav-link", href: "#home", "Home" }
                        }
                        li {
                            a { class: "mobile-nav-link", href: "#about", "About" }
                        }
                        li {
                            a { class: "mobile-nav-link", href: "#projects", "Projects" }
                        }
                        li {
                            a { class: "mobile-nav-link", href: "#contact", "Contact" }
                        }
                    }
                }
                div { class: "mobile-theme-switcher",
                    h3 { "Theme" }
                    div { class: "mobile-theme-options",
                        button { class: "theme-option", "data-theme": "mocha",
                            span { class: "color-preview mocha" }
                            span { "Mocha" }
                        }
                        button { class: "theme-option", "data-theme": "macchiato",
                            span { class: "color-preview macchiato" }
                            span { "Macchiato" }
                        }
                        button { class: "theme-option", "data-theme": "latte",
                            span { class: "color-preview latte" }
                            span { "Latte" }
                        }
                        button { class: "theme-option", "data-theme": "rosepine",
                            span { class: "color-preview rosepine" }
                            span { "Rose Pine" }
                        }
                    }
                }
                div { class: "mobile-social-links",
                    a { aria_label: "GitHub", href: "#",
                        i { class: "fab fa-github" }
                    }
                    a { aria_label: "LinkedIn", href: "#",
                        i { class: "fab fa-linkedin" }
                    }
                    a { aria_label: "Twitter", href: "#",
                        i { class: "fab fa-twitter" }
                    }
                }
            }
        }
    }
}
