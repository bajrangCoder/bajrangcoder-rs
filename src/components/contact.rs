use crate::config::PERSONAL_DATA;
use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    rsx! {
        section { class: "contact", id: "contact",
            div { class: "container",
                h2 { class: "section-title",
                    span { class: "comment", "// " }
                    "Contact Me"
                }
                div { class: "contact-content",
                    div { class: "contact-info",
                        h3 { "Get In Touch" }
                        p {
                            "Feel free to reach out if you're looking for a developer, have a question, or just want to connect."
                        }
                        div { class: "contact-details",
                            div { class: "contact-item",
                                div { class: "contact-icon",
                                    svg {
                                        fill: "none",
                                        height: "24",
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        view_box: "0 0 24 24",
                                        width: "24",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        path { d: "M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z" }
                                    }
                                }
                                div { class: "contact-text",
                                    h4 { "Phone" }
                                    p { "{PERSONAL_DATA.phone}" }
                                }
                            }
                            div { class: "contact-item",
                                div { class: "contact-icon",
                                    svg {
                                        fill: "none",
                                        height: "24",
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        view_box: "0 0 24 24",
                                        width: "24",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        path { d: "M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z" }
                                        polyline { points: "22,6 12,13 2,6" }
                                    }
                                }
                                div { class: "contact-text",
                                    h4 { "Email" }
                                    p { "{PERSONAL_DATA.email}" }
                                }
                            }
                            div { class: "contact-item",
                                div { class: "contact-icon",
                                    svg {
                                        fill: "none",
                                        height: "24",
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        view_box: "0 0 24 24",
                                        width: "24",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        path { d: "M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0 1 18 0z" }
                                        circle { cx: "12", cy: "10", r: "3" }
                                    }
                                }
                                div { class: "contact-text",
                                    h4 { "Location" }
                                    p { "{PERSONAL_DATA.address}" }
                                }
                            }
                        }
                        div { class: "social-links",
                            a { aria_label: "GitHub", href: "{PERSONAL_DATA.github}",
                                i { class: "fab fa-github" }
                            }
                            a { aria_label: "LinkedIn", href: "{PERSONAL_DATA.linkedin}",
                                i { class: "fab fa-linkedin" }
                            }
                            a { aria_label: "Twitter", href: "{PERSONAL_DATA.twitter}",
                                i { class: "fab fa-twitter" }
                            }
                        }
                    }
                    div { class: "contact-form-container",
                        div { class: "form-decoration",
                            div { class: "form-shape" }
                            div { class: "form-shape" }
                            div { class: "form-shape" }
                        }
                        form { class: "contact-form", id: "contact-form",
                            div { class: "form-group",
                                label { r#for: "name", "Name" }
                                div { class: "input-container",
                                    svg {
                                        fill: "none",
                                        height: "18",
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        view_box: "0 0 24 24",
                                        width: "18",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        path { d: "M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" }
                                        circle { cx: "12", cy: "7", r: "4" }
                                    }
                                    input {
                                        id: "name",
                                        name: "name",
                                        r#type: "text",
                                        required: "false",
                                    }
                                }
                            }
                            div { class: "form-group",
                                label { r#for: "email", "Email" }
                                div { class: "input-container",
                                    svg {
                                        fill: "none",
                                        height: "18",
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        view_box: "0 0 24 24",
                                        width: "18",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        path { d: "M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z" }
                                        polyline { points: "22,6 12,13 2,6" }
                                    }
                                    input {
                                        id: "email",
                                        name: "email",
                                        r#type: "email",
                                        required: "false",
                                    }
                                }
                            }
                            div { class: "form-group",
                                label { r#for: "subject", "Subject" }
                                div { class: "input-container",
                                    svg {
                                        fill: "none",
                                        height: "18",
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        view_box: "0 0 24 24",
                                        width: "18",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        line {
                                            x1: "4",
                                            x2: "20",
                                            y1: "9",
                                            y2: "9",
                                        }
                                        line {
                                            x1: "4",
                                            x2: "20",
                                            y1: "15",
                                            y2: "15",
                                        }
                                        line {
                                            x1: "10",
                                            x2: "8",
                                            y1: "3",
                                            y2: "21",
                                        }
                                        line {
                                            x1: "16",
                                            x2: "14",
                                            y1: "3",
                                            y2: "21",
                                        }
                                    }
                                    input {
                                        id: "subject",
                                        name: "subject",
                                        r#type: "text",
                                        required: "false",
                                    }
                                }
                            }
                            div { class: "form-group",
                                label { r#for: "message", "Message" }
                                div { class: "input-container textarea",
                                    svg {
                                        fill: "none",
                                        height: "18",
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        view_box: "0 0 24 24",
                                        width: "18",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        path { d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" }
                                        "  stroke-linejoin=\"round\">"
                                        path { d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" }
                                    }
                                    textarea {
                                        id: "message",
                                        name: "message",
                                        required: "false",
                                        rows: "5",
                                    }
                                }
                            }
                            button { class: "btn primary", r#type: "submit", "Send Message" }
                        }
                    }
                }
            }
        }
    }
}
