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
    let window_clone = window.clone();
    let window_clone1 = window.clone();

    rsx! {
        document::Link { rel: "stylesheet", href: HEADER_CSS }
        header {
            onmousedown: move |_| {
                if let Err(e) = window.drag_window() {
                    tracing::error!("Failed to drag window: {}", e);
                }
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
                    onmousedown: move |event| {
                        event.stop_propagation();
                        tracing::info!("DataViewer button mousedown");
                    },
                    onclick: move |event| {
                        event.stop_propagation();
                        navigator.push("/");
                    },
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
                    }
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
                            tracing::info!("Minimize button mousedown");
                        },
                        onclick: move |_| {
                            window_clone.set_minimized(true)
                        },
                        img {
                            src: MINIMIZE_SVG
                        }
                    },
                    div {
                        class: "element",
                        onmousedown: move |event| {
                            event.stop_propagation();
                            tracing::info!("Close button mousedown");
                        },
                        onclick: move |_| {
                            window_clone1.close()
                        },
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