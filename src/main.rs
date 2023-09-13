#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        style { include_str!("../src/bulma.min.css") },
        div {
            class: "container is-fluid",
            //style { include_str!("../src/style.css") },
            div {
                h3 {
                    class:"title is-4 is-spaced bd-anchor-title",
                    "Arcdps Settings"
                },
                div {
                    class: "field",
                    label {
                        class:"label",
                        "Arc DPS Log folder"
                    },
                    div {
                        class: "file has-name",
                        id: "folder",
                        label {
                            class: "file-label",
                            input {
                                r#type: "file",
                                name: "arcdpsFolder",
                                class: "file-input",
                                directory: true,
                                onchange: |evt| {
                                    if let Some(file_engine) = &evt.files {
                                        let files = file_engine.files();
                                        for file_name in &files {
                                            println!("{}", file_name);
                                            // Do something with the folder path
                                        }
                                    }
                                }
                            }
                            span {
                                class: "file-cta",
                                span {
                                    class: "file-icon",
                                    i {
                                        class: "fas fa-upload",
                                    }
                                }
                                span {
                                    class: "file-label",
                                    "Browse"
                                }
                            }
                            span {
                                class: "file-name",
                                "Arc DPS Log Directory"
                            }
                        }
                    }
                },
                div {
                    class: "field",
                    label {
                        class:"label",
                        "Arc DPS Log folder"
                    },
                    div {
                        class: "control",
                        input {
                            class:"input",
                            placeholder: "Arc DPS Folder",
                            value: "Test/Test/args"
                        }
                    }
                },
                div {
                    class: "field",
                    label {
                        class:"label",
                        "DPS Report API URL"
                    },
                    div {
                        class: "control",
                        input {
                            class:"input",
                            placeholder: "Bot API URL",
                            value: "https://Test/api"
                        }
                    }
                },
                div {
                    button {
                        class:"button is-primary",
                        "Save & Exit"
                    }
                }
            },
            script { include_str!("../src/input.js") },

        }
    })
}
