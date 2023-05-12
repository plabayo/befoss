use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        header {
            div (class = "nav") {
                div (class = "nav-title") {
                    a (href="/") {
                        h1 {
                            span (style = "color: black;") { "#be" }
                            span (style = "color: yellow;") { "foss" }
                            span (style = "color: red;") { ".dev" }
                        }
                    }
                }
                div (class = "nav-links") {
                    a (href="/proposal") { "🪧 Proposal" }
                    a (href="/game") { "🎮 Game" }
                }
            }
            div (class = "flag") {
                div (style = "background-color: black;") {}
                div (style = "background-color: yellow;") {}
                div (style = "background-color: red;") {}
            }
        }
        main {
            div(class = "container") {
                div {
                    p {
                        "👋 Hello Fellow Citizens, let's talk about software 👩‍💻"
                    }
                    img (
                        src=".perseus/static/burning_money.png",
                        alt="Politicians Burning Our Tax Money on Shitware",
                        style="width: 95%;",
                        class="center-img"
                    ) {}
                }
            }
        }
        footer {
            p {
                "A "
                a(href = "https://github.com/plabayo/befoss") {
                    "Free and Open Source"
                }
                " proposal by "
                a (href="https://plabayo.tech") {
                    "Plabayo.tech"
                }
                "."
            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "#befossdev" }
        meta(charset = "UTF-8")
        meta(name = "viewport", content = "width=device-width, initial-scale=1.0")
        link(rel = "stylesheet", href = ".perseus/static/style.css")
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}