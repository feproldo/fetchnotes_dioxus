use dioxus::{desktop::{Config, LogicalSize, WindowBuilder}, prelude::*};

use components::AppHeader;
use views::*;

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(AppHeader)]
    #[route("/")]
    Home {},
    #[route("/api")]
    Api {}
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styles/main.css");

fn main() {
    dioxus::LaunchBuilder::new().with_cfg(
Config::default()
            .with_menu(None)
            .with_window(
                WindowBuilder::new()
                    .with_decorations(false)
                    .with_inner_size(LogicalSize::new(1400, 900))
                    
            ))
        .launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }


        Router::<Route> {}
    }
}
