use crate::config::PERSONAL_DATA;
use dioxus::prelude::*;
use gloo_timers::future::sleep;
use std::time::Duration;

const SKILLS: &str = "\n{\n  \"frontend\": [\"React\", \"Vue\", \"TypeScript\", \"Tailwind\"],\n  \"backend\": [\"Node.js\", \"Express\", \"Python\", \"Django\"],\n  \"database\": [\"MongoDB\", \"PostgreSQL\", \"Redis\"],\n  \"devops\": [\"Docker\", \"AWS\", \"CI/CD\", \"Kubernetes\"]\n}";

#[component]
pub fn Hero() -> Element {
    const WORDS: &[&str] = &[
        "modern web applications",
        "low level programs",
        "scalable backends",
        "elegant solutions",
        "desktop applications",
        "ai, llm",
    ];

    let mut word_index = use_signal(|| 0);
    let mut char_index = use_signal(|| 0);
    let mut is_deleting = use_signal(|| false);
    let mut text = use_signal(|| String::new());

    use_future(move || async move {
        loop {
            // Use WORDS instead of words
            let current_word = WORDS[word_index()];
            let typing_speed = if is_deleting() {
                Duration::from_millis(50)
            } else {
                Duration::from_millis(100)
            };

            if is_deleting() {
                // Remove a character
                if char_index() > 0 {
                    char_index -= 1;
                    text.set(current_word[..(char_index() as usize)].to_string());
                } else {
                    // Move to next word
                    is_deleting.set(false);
                    word_index.set((word_index() + 1) % WORDS.len());
                    sleep(Duration::from_millis(500)).await;
                    continue;
                }
            } else {
                // Add a character
                if char_index() < current_word.len() {
                    char_index += 1;
                    text.set(current_word[..(char_index() as usize)].to_string());
                } else {
                    // Word is complete, pause before next word
                    is_deleting.set(true);
                    sleep(Duration::from_millis(1000)).await;
                    continue;
                }
            }

            sleep(typing_speed).await;
        }
    });

    rsx! {
        section { id: "home", class: "hero",
            div { class: "container",
                div { class: "hero-content",
                    div { class: "hero-text",
                        div { class: "hero-badge", "{PERSONAL_DATA.designation}" }
                        h1 {
                            span { class: "comment", "// Hello, I'm" }
                            br {}
                            "{PERSONAL_DATA.name}"
                        }
                        div { class: "hero-description",
                            div { class: "typing-wrapper",
                                p {
                                    "I build "
                                    span { class: "typing-text", "{text}" }
                                }
                                div { class: "typing-cursor" }
                            }
                            p { class: "hero-subtitle",
                                "I'm passionate about creating innovative solutions that solve real-world problems."
                            }
                        }
                        div { class: "hero-buttons",
                            a { href: "#projects", class: "btn primary", "View Projects" }
                            a { href: "#contact", class: "btn secondary", "Contact Me" }
                        }
                    }
                }
                div { class: "terminal-container",
                    div { class: "terminal",
                        div { class: "terminal-header",
                            div { class: "terminal-buttons",
                                span { class: "terminal-button close" }
                                span { class: "terminal-button minimize" }
                                span { class: "terminal-button maximize" }
                            }
                            div { class: "terminal-title", "alex@devfolio: ~/projects" }
                            div { class: "terminal-actions",
                                i { class: "fas fa-minus" }
                                i { class: "fas fa-expand" }
                            }
                        }
                        div { class: "terminal-body",
                            div { class: "line",
                                span { class: "prompt", "alex@devfolio:~$" }
                                span { class: "command", "cd projects" }
                            }
                            div { class: "line",
                                span { class: "prompt", "alex@devfolio:~/projects$" }
                                span { class: "command", "ls -la" }
                            }
                            div { class: "line output",
                                span { "drwxr-xr-x 2 alex alex 4096 Mar 15 14:30 ." }
                            }
                            div { class: "line output",
                                span { "drwxr-xr-x 18 alex alex 4096 Mar 15 14:28 .." }
                            }
                            div { class: "line output",
                                span {
                                    "-rw-r--r-- 1 alex alex 8980 Mar 10 09:15\n                    e-commerce.js"
                                }
                            }
                            div { class: "line output",
                                span {
                                    "-rw-r--r-- 1 alex alex 5240 Feb 28 16:42\n                    portfolio.css"
                                }
                            }
                            div { class: "line output",
                                span {
                                    "-rw-r--r-- 1 alex alex 3540 Mar 05 11:32\n                    api-service.ts"
                                }
                            }
                            div { class: "line",
                                span { class: "prompt", "alex@devfolio:~/projects$" }
                                span { class: "command", "cat skills.json" }
                            }
                            div { class: "line output code-block",
                                pre { "{SKILLS}" }
                            }
                            div { class: "line",
                                span { class: "prompt", "alex@devfolio:~/projects$" }
                                span { class: "command blink", "_" }
                            }
                        }
                    }
                }
            }
        }
    }
}
