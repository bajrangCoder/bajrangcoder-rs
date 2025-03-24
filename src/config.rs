#[allow(dead_code)]

pub struct PersonalInfo {
    pub name: &'static str,
    pub designation: &'static str,
    pub description: &'static str,
    pub email: &'static str,
    pub phone: &'static str,
    pub address: &'static str,
    pub github: &'static str,
    pub bluesky: &'static str,
    pub linked_in: &'static str,
    pub twitter: &'static str,
    pub youtube: &'static str,
    pub hugging_face: &'static str,
    pub dev_username: &'static str,
    pub resume: &'static str,
}

pub const PERSONAL_DATA: PersonalInfo = PersonalInfo {
    name: "Raunak",
    designation: "Software Developer",
    description: "My name is Raunak Raj. I am a professional and enthusiastic programmer in my daily life. I am a quick learner with a self-learning attitude. I love to learn and explore new technologies and I'm passionate about problem-solving and innovation. My core skills are based on JavaScript, Rust, Python, and I love to do most things using Rust and JS. My skills and technologies adapt based on the situation and need. I am available for any kind of work opportunity that suits my skills and interests.",
    email: "bajrangcoders@gmail.com",
    phone: "Null",
    address: "Bihar, India",
    github: "https://github.com/bajrangCoder",
    bluesky: "https://bsky.app/profile/bajrangcoder.dev",
    linked_in: "https://www.linkedin.com/in/raunak-raj-603196261",
    twitter: "https://twitter.com/bajrangCoder",
    youtube: "BajrangCoder",
    hugging_face: "https://huggingface.co/bajrangCoder",
    dev_username: "bajrangCoder",
    resume: "#"
};

#[derive(Clone, PartialEq)]
pub struct Theme {
    pub name: &'static str,
    pub id: &'static str,
    pub preview_class: &'static str,
}

pub const THEMES: &[Theme] = &[
    Theme {
        name: "Catppuccin Mocha",
        id: "mocha",
        preview_class: "mocha",
    },
    Theme {
        name: "Macchiato",
        id: "macchiato",
        preview_class: "macchiato",
    },
    Theme {
        name: "Catppuccin Latte",
        id: "latte",
        preview_class: "latte",
    },
    Theme {
        name: "Rose Pine",
        id: "rosepine",
        preview_class: "rosepine",
    },
];

// skills

pub const LANGUAGES: &[&str] = &["Rust", "Python", "JavaScript", "Bash", "TypeScript"];
pub const FRAMEWORKS_AND_TOOLS: &[&str] = &[
    "SCSS", "Dioxus", "Svelte", "React", "Burn", "Linux", "Zed", "more",
];
