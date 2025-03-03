use dioxus::prelude::*;
const CSS: Asset = asset!("/assets/styles/pages/home.css");
#[component]
pub fn Home() -> Element {
    rsx! {
        document::Link { rel: "Stylesheet", href: CSS },
        main { 
            h1 {
                "Welcome to your workspace!"
            },
            h2 {
                "Lets start! Choose tool"
            }
        }
    }
}
