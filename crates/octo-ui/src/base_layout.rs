#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BaseLayoutProps {
    pub title: String,
    pub fav_icon_src: Option<String>,
    pub web_assembly: Option<(String, String)>,
    pub stylesheets: Vec<String>,
    pub js_href: Option<String>,
    pub header_left: Element,
    pub header_right: Option<Element>,
    pub children: Element,
    pub sidebar: Element,
    pub sidebar_footer: Element,
    pub sidebar_header: Element,
}

pub fn BaseLayout(props: BaseLayoutProps) -> Element {
    let hydrate: Option<String> = if let Some((js, wasm)) = props.web_assembly {
        Some(format!(
            "import init, {{ hydrate }} from '{}'; await init('{}'); hydrate(); document.addEventListener('mu:after-render', () => {{ hydrate(); }});",
            js, wasm
        ))
    } else {
        None
    };

    rsx!(
        head {
            title {
                "{props.title}"
            }
            meta {
                charset: "utf-8"
            }
            meta {
                "http-equiv": "X-UA-Compatible",
                content: "IE=edge"
            }
            meta {
                name: "viewport",
                content: "width=device-width, initial-scale=1"
            }
            for href in &props.stylesheets {
                link {
                    rel: "stylesheet",
                    href: "{href}",
                    "type": "text/css"
                }
            }
            script {
                src: "https://unpkg.com/@digicreon/mujs@1.4.3/dist/mu.min.js"
            }
            script {
                dangerous_inner_html: "mu.init();"
            }
            if let Some(js_href) = props.js_href {
                script {
                    "type": "module",
                    src: "{js_href}"
                }
            }
            if let Some(fav_icon_src) = props.fav_icon_src {
                link {
                    rel: "icon",
                    "type": "image/svg+xml",
                    href: "{fav_icon_src}"
                }
            }
            if let Some(hydrate) = hydrate {
                script {
                    "type": "module",
                    dangerous_inner_html: hydrate
                }
            }
        }
        body {
            div {
                class: "flex h-screen overflow-hidden",
                nav {
                    id: "sidebar",
                    class: "
                        border-r border-base-300
                        fixed
                        bg-base-200
                        inset-y-0
                        left-0
                        w-64
                        transform
                        -translate-x-full
                        transition-transform
                        duration-200
                        ease-in-out
                        flex
                        flex-col
                        lg:translate-x-0
                        lg:static
                        lg:inset-auto
                        lg:transform-none
                        z-20",
                    div {
                        class: "flex items-center p-4",
                        {props.sidebar_header}
                    }
                    div {
                        class: "flex-1 overflow-y-auto",
                        {props.sidebar}
                    }
                    div {
                        class: "p-4",
                        {props.sidebar_footer}
                    }
                }
                main {
                    id: "main-content",
                    class: "flex-1 flex flex-col",
                    header {
                        class: "flex items-center gap-3 p-4 border-b border-base-300",
                        button {
                            id: "toggleButton",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                width: "24",
                                height: "24",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                class: "lucide lucide-panel-left",
                                rect {
                                    width: "18",
                                    height: "18",
                                    x: "3",
                                    y: "3",
                                    rx: "2",
                                }
                                path {
                                    d: "M9 3v18",
                                }
                            }
                        }
                        div {
                            class: "w-full min-w-0 flex items-center justify-between gap-4",
                            div {
                                class: "min-w-0",
                                {props.header_left}
                            }
                            if let Some(header_right) = props.header_right {
                                div {
                                    class: "shrink-0",
                                    {header_right}
                                }
                            }
                        }
                    }
                    section {
                        class: "flex-1 overflow-y-auto",
                        {props.children}
                    }
                }
            }
        }
    )
}
