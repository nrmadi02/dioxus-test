#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let mut name = use_signal(|| "bob".to_string());

    rsx! {
        div{
             class: "container mx-auto",
             Link {
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }
        div {
            h1 { "High-Five counter: {count}" }
            div {
                class: "flex items-center mt-2 gap-3",
                Button { text: "Up high!", onclick:move |_| count += 1 }
                Button { text: "Down low!", onclick:move |_| count -= 1 }
            }
                div {
                    class: "flex items-center mt-2 gap-3",
                   input {
                          value: "{name}",
                       oninput: move |event| name.set(event.value())
             }
                }
            }
        }

    }
}
#[derive(PartialEq, Props, Clone)]
struct ButtonProps {
    text: String,
    onclick: EventHandler<MouseEvent>,
}

#[component]
fn Button(props: ButtonProps) -> Element {
    rsx! {
        button { class: "text-red-500 px-4 py-2 rounded bg-white ring-2 ring-red-500 focus:outline-none", onclick: move |event| props.onclick.call(event), "{props.text}" }
    }
}
