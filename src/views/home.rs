use dioxus::prelude::*;
const CSS: &str = include_str!("../../assets/styles/pages/home.css");
#[component]
pub fn Home() -> Element {
    rsx! {
        style { "{CSS}" },
        main { 
            h1 {
                "Welcome to your workspace!"
            }
            h2 {
                "Lets start! Choose tool"
            }
        }
    }
}
