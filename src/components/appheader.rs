use crate::Route;
use dioxus::{desktop::use_window, logger::tracing, prelude::*};

const HEADER_CSS: Asset = asset!("/assets/styles/header.css");
const SERVER_SVG: Asset = asset!("/assets/svgs/server.svg");
const DATABASE_SVG: Asset = asset!("/assets/svgs/database.svg");
const TERMINAL_SVG: Asset = asset!("/assets/svgs/terminal.svg");
const SETTINGS_SVG: Asset = asset!("/assets/svgs/settings.svg");
const CLOSE_SVG: Asset = asset!("/assets/svgs/close.svg");
const MINIMIZE_SVG: Asset = asset!("/assets/svgs/minimize.svg");

#[component]
pub fn AppHeader() -> Element {
    let navigator = use_navigator();
    let window = use_window();
    let windowclone1 = use_window();
    let windowclone2 = use_window();

    rsx! {
        document::Link { rel: "stylesheet", href: HEADER_CSS }
        header {
            onmousedown: move |_| {
                let _ = &window.drag_window();
                tracing::info!("Header clicked");
            },
            div {
                class: "left",
                div {
                    onmousedown: move |event| {
                        event.stop_propagation();
                        tracing::info!("API button mousedown");
                    },
                    onclick: move |event| {
                        event.stop_propagation();
                        navigator.push("/api");
                        tracing::info!("API button clicked");
                    },
                    class: "element",
                    img {
                        src: SERVER_SVG
                    }
                },
                div {
                    onmousedown: move |event| {
                        event.stop_propagation();
                        tracing::info!("DataViewer button mousedown");
                    },
                    onclick: move |event| {
                        event.stop_propagation();
                        navigator.push("/dataviewer");
                        tracing::info!("DataViewer button clicked");
                    },
                    class: "element",
                    img {
                        src: DATABASE_SVG
                    }
                }
            },
            div {
                class: "center",
                span {
                    "fetcher"
                }
            },
            div {
                class: "right",
                div {
                    class: "settings",
                    div {
                        class: "element",
                        img {
                            src: TERMINAL_SVG
                        }
                    },
                    div {
                        class: "element",
                        img {
                            src: SETTINGS_SVG
                        }
                    }
                },
                div {
                    class: "controls",
                    div {
                        class: "element",
                        onmousedown: move |event| {
                            event.stop_propagation();
                            tracing::info!("DataViewer button mousedown");
                        },
                        onclick: move |_| {
                            windowclone1.set_minimized(true);
                        },
                        img {
                            src: MINIMIZE_SVG
                        }
                    },
                    div {
                        onmousedown: move |event| {
                            event.stop_propagation();
                            tracing::info!("DataViewer button mousedown");
                        },
                        onclick: move |_| {
                            windowclone2.close();
                        },
                        class: "element",
                        img {
                            src: CLOSE_SVG
                        }
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}