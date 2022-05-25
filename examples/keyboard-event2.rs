use bevy::{log::LogPlugin, prelude::*};
use bevy_dioxus::desktop::prelude::*;
use dioxus::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Keyboard Event".to_string(),
            ..Default::default()
        })
        .add_plugin(LogPlugin)
        .add_plugin(DioxusPlugin::<(), ()>::new(Root))
        // .add_startup_system(setup)
        // .add_system(handle_core_cmd)
        // .add_system(log_keyboard_event)
        .run();
}

// UI Component
#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            tabindex: "0",
            style: "width: 100vw; height: 100vh; background: red;",
            onkeydown: move |e| {
                println!("onkeydown: {:#?}", e);
            }
        }
    })
}
