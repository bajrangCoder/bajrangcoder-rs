use crate::{
    config::PERSONAL_DATA,
    icons::{Github, Hash, Linkedin, Mail, MapPin, Message, Phone, Twitter, User},
};
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
                                div { class: "contact-icon", Phone {} }
                                div { class: "contact-text",
                                    h4 { "Phone" }
                                    p { "{PERSONAL_DATA.phone}" }
                                }
                            }
                            div { class: "contact-item",
                                div { class: "contact-icon", Mail {} }
                                div { class: "contact-text",
                                    h4 { "Email" }
                                    p { "{PERSONAL_DATA.email}" }
                                }
                            }
                            div { class: "contact-item",
                                div { class: "contact-icon", MapPin {} }
                                div { class: "contact-text",
                                    h4 { "Location" }
                                    p { "{PERSONAL_DATA.address}" }
                                }
                            }
                        }
                        div { class: "social-links",
                            a {
                                aria_label: "GitHub",
                                href: "{PERSONAL_DATA.github}",
                                Github {}
                            }
                            a {
                                aria_label: "LinkedIn",
                                href: "{PERSONAL_DATA.linkedin}",
                                Linkedin {}
                            }
                            a {
                                aria_label: "Twitter",
                                href: "{PERSONAL_DATA.twitter}",
                                Twitter {}
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
                                    User {}
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
                                    Mail {}
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
                                    Hash {}
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
                                    Message {}
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
