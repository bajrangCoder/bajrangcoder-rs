use crate::config::{FRAMEWORKS_AND_TOOLS, LANGUAGES, PERSONAL_DATA};
use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        section { id: "about", class: "about",
            div { class: "container",
                h2 { class: "section-title",
                    span { class: "comment", "// " }
                    "About Me"
                }
                div { class: "about-content",
                    div { class: "about-text",
                        p { "{PERSONAL_DATA.description}" }

                        h3 { "Skills" }
                        div { class: "skills-container",
                            div { class: "skill-category",
                                h4 { "Programing Languages" }
                                div { class: "skills-grid",
                                    {
                                        LANGUAGES
                                            .iter()
                                            .map(|lang| {
                                                rsx! {
                                                    div { class: "skill-tag", "{lang}" }
                                                }
                                            })
                                    }
                                }
                            }
                            div { class: "skill-category",
                                h4 { "Frameworks & Tools" }
                                div { class: "skills-grid",
                                    {
                                        FRAMEWORKS_AND_TOOLS
                                            .iter()
                                            .map(|lang| {
                                                rsx! {
                                                    div { class: "skill-tag", "{lang}" }
                                                }
                                            })
                                    }
                                }
                            }
                        }

                        div { class: "experience-timeline",
                            h4 { "Experience" }
                            div { class: "timeline",
                                div { class: "timeline-item",
                                    div { class: "timeline-dot" }
                                    div { class: "timeline-content",
                                        h5 { "Software Engineer" }
                                        p { class: "company", "FOSS & OS." }
                                        p { class: "timeline-date", "2022 - Present" }
                                    }
                                }
                            }
                        }

                        div { class: "about-buttons",
                            a { class: "btn primary", href: "#", "Download CV" }
                            a { class: "btn secondary", href: "#contact", "Get in Touch" }
                        }
                    }
                }
            }
        }
    }
}
