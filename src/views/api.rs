use dioxus::prelude::*;
const CSS: Asset = asset!("/assets/styles/pages/api.css");
#[component]
pub fn Api() -> Element {
    rsx! {
        document::Link { rel: "Stylesheet", href: CSS },
        main { 
            h1 {
                "TODO"
            }
        }
    }
}
