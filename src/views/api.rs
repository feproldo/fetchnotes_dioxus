use dioxus::prelude::*;
const CSS: &str = include_str!("../../assets/styles/pages/api.css");
#[component]
pub fn Api() -> Element {
    rsx! {
        style { "{CSS}" },
        main {
            div {
                class: "container",
                Left {}
                Right {}
            }
        }
    }
}

#[component]
pub fn Left() -> Element {
    rsx! {
        div {
            class: "left",
            h1 {
                "⚠️ WORK IN PROGRESS ⚠️"
            }
        }
        
    }
}

#[component]
pub fn Right() -> Element {
    rsx! {
        div {
            class: "right",
            h1 {
                "hello test"
            }
        }
    }
}
