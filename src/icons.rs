#![allow(unused, non_snake_case)]

use dioxus::prelude::*;

pub(crate) fn Github() -> Element {
    rsx! {
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
            path { d: "M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4" }
            path { d: "M9 18c-4.51 2-5-2-7-2" }
        }
    }
}
pub(crate) fn Linkedin() -> Element {
    rsx! {
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
}
pub(crate) fn Twitter() -> Element {
    rsx! {
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
            path { d: "M22 4s-.7 2.1-2 3.4c1.6 10-9.4 17.3-18 11.6 2.2.1 4.4-.6 6-2C3 15.5.5 9.6 3 5c2.2 2.6 5.6 4.1 9 4-.9-4.2 4-6.6 7-3.8 1.1 0 3-1.2 3-1.2z" }
        }
    }
}
pub(crate) fn Phone() -> Element {
    rsx! {
        svg {
            height: "24",
            width: "24",
            fill: "none",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            xmlns: "http://www.w3.org/2000/svg",
            path { d: "M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z" }
        }
    }
}
pub(crate) fn Mail() -> Element {
    rsx! {
        svg {
            height: "18",
            width: "18",
            fill: "none",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            xmlns: "http://www.w3.org/2000/svg",
            rect {
                height: "16",
                rx: "2",
                width: "20",
                x: "2",
                y: "4",
            }
            path { d: "m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" }
        }
    }
}
pub(crate) fn MapPin() -> Element {
    rsx! {
        svg {
            fill: "none",
            height: "24",
            width: "24",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            xmlns: "http://www.w3.org/2000/svg",
            path { d: "M20 10c0 4.993-5.539 10.193-7.399 11.799a1 1 0 0 1-1.202 0C9.539 20.193 4 14.993 4 10a8 8 0 0 1 16 0" }
            circle { cx: "12", cy: "10", r: "3" }
        }
    }
}
pub(crate) fn Palette() -> Element {
    rsx! {
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
            circle {
                cx: "13.5",
                cy: "6.5",
                fill: "currentColor",
                r: ".5",
            }
            circle {
                cx: "17.5",
                cy: "10.5",
                fill: "currentColor",
                r: ".5",
            }
            circle {
                cx: "8.5",
                cy: "7.5",
                fill: "currentColor",
                r: ".5",
            }
            circle {
                cx: "6.5",
                cy: "12.5",
                fill: "currentColor",
                r: ".5",
            }
            path { d: "M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.835-.437-1.125-.29-.289-.438-.652-.438-1.125a1.64 1.64 0 0 1 1.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.554C21.965 6.012 17.461 2 12 2z" }
        }
    }
}
pub(crate) fn User() -> Element {
    rsx! {
        svg {
            fill: "none",
            height: "18",
            width: "18",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            xmlns: "http://www.w3.org/2000/svg",
            path { d: "M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2" }
            circle { cx: "12", cy: "7", r: "4" }
        }
    }
}
pub(crate) fn Hash() -> Element {
    rsx! {
        svg {
            fill: "none",
            height: "18",
            width: "18",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
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
    }
}
pub(crate) fn Message() -> Element {
    rsx! {
        svg {
            fill: "none",
            height: "18",
            width: "18",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            xmlns: "http://www.w3.org/2000/svg",
            path { d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" }
        }
    }
}
pub(crate) fn Minus() -> Element {
    rsx! {
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
            path { d: "M5 12h14" }
        }
    }
}
pub(crate) fn Maximize() -> Element {
    rsx! {
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
            path { d: "M8 3H5a2 2 0 0 0-2 2v3" }
            path { d: "M21 8V5a2 2 0 0 0-2-2h-3" }
            path { d: "M3 16v3a2 2 0 0 0 2 2h3" }
            path { d: "M16 21h3a2 2 0 0 0 2-2v-3" }
        }
    }
}
