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
                    a (href="/assets") { "🎨 Assets" }
                    a (href="/swag") { "👕 Swag" }
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
                }
            }
        }
        footer {
            p {
                "A Free and Open Source project actively researched and developed by "
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
        title { "Welcome to Perseus!" }
        style {
            (r#"
                body {
                    width: 100vw;
                    height: 100vh;
                    margin: 0;
                    padding: 0;
                    box-sizing: content-box;
                }

                div#root {
                    height: 100%;
                    display: grid;
                    grid-template-columns: 1fr;
                    grid-template-rows: auto 1fr auto;
                    grid-template-areas: 
                        'header'
                        'main'
                        'footer';
                }

                header {
                    grid-area: header;
                }

                main {
                    grid-area: main;
                }

                footer {
                    grid-area: footer;
                }

                footer p {
                    text-align: center;
                    padding: 0 10px;
                }

                header, footer {
                    background-color: #bbb;
                }

                header a, header a:visited {
                    text-decoration: none;
                }

                header .nav {
                    display: flex;
                    justify-content: space-between;
                    padding: 0 10px;
                }

                header .nav-links {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                }

                header .nav-links a {
                    padding: 0 10px;
                    font-size: 1.2rem;
                    color: black;
                    font-weight: bold;
                }

                header div.flag {
                    height: 5px;
                    display: flex;
                    flex-direction: row;
                }

                header div.flag > div {
                    flex: 1;
                }

                main {
                    background-color: #69707f;
                }

                .container {
                    max-width: 780px;
                    margin: auto;
                    background-color: #eee;
                    height: 100%;
                }

                .container p {
                    margin: 0;
                    padding: 10px;
                }
            "#)
        }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}