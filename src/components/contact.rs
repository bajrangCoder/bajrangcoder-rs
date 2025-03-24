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
                            a { aria_label: "GitHub", href: "#",
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
                                    path { d: "M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22" }
                                }
                            }
                            a { aria_label: "LinkedIn", href: "#",
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
                                    path { d: "M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z" }
                                    rect {
                                        height: "12",
                                        width: "4",
                                        x: "2",
                                        y: "9",
                                    }
                                    circle { cx: "4", cy: "4", r: "2" }
                                }
                            }
                            a { aria_label: "Twitter", href: "#",
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
                                    path { d: "M23 3a10.9 10.9 0 0 1-3.14 1.53 4.48 4.48 0 0 0-7.86 3v1A10.66 10.66 0 0 1 3 4s-4 9 5 13a11.64 11.64 0 0 1-7 2c9 5 20 0 20-11.5a4.5 4.5 0 0 0-.08-.83A7.72 7.72 0 0 0 23 3z" }
                                }
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
