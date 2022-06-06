use bevy::{log::LogPlugin, prelude::*, window::RequestRedraw};
use bevy_dioxus::desktop::prelude::*;
use dioxus::{fermi::Readable, prelude::*};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Counter".to_string(),
            ..Default::default()
        })
        .add_plugin(LogPlugin)
        .add_plugin(DioxusPlugin::<CoreCommand, ()>::new(Root))
        .add_startup_system(setup)
        .add_system(handle_core_cmd.label("handle-core-cmd"))
        .add_system(update_count_atom.after("handle-core-cmd"))
        .run();
}

#[derive(Component, Default, Clone)]
pub struct Count(pub u32);

pub static COUNT: Atom<Count> = |_| Count(100);

#[derive(Clone, Debug)]
enum CoreCommand {
    Increment,
    Decrement,
    Reset,
}

fn setup(mut commands: Commands) {
    info!("🧠 Spawn count");
    commands.spawn().insert(Count::default());
}

// TODO: should be derived by macro
fn update_count_atom(query: Query<&Count, Changed<Count>>, vdom_tx: Res<Sender<VDomCommand>>) {
    for count in query.iter() {
        info!("🧠 Counter Changed: {}", count.0);
        match vdom_tx.try_send(VDomCommand::GlobalState(GlobalState::new(
            COUNT.unique_id() as usize,
            Box::new(count.clone()),
        ))) {
            Ok(()) => {}
            Err(e) => match e {
                TrySendError::Full(e) => {
                    error!(
                        "Failed to send VDomCommand: channel is full: event: {:?}",
                        e
                    );
                }
                TrySendError::Closed(e) => {
                    error!(
                        "Failed to send VDomCommand: channel is closed: event: {:?}",
                        e
                    );
                }
            },
        }
    }
}

fn handle_core_cmd(mut events: EventReader<CoreCommand>, mut query: Query<&mut Count>) {
    for cmd in events.iter() {
        info!("core cmd");
        let mut count = query.single_mut();
        match cmd {
            CoreCommand::Increment => {
                info!("🧠 Increment");
                count.0 += 1;
            }
            CoreCommand::Decrement => {
                if count.0 > 0 {
                    info!("🧠 Decrement");
                    count.0 -= 1;
                }
            }
            CoreCommand::Reset => {
                if count.0 != 0 {
                    info!("🧠 Reset");
                    count.0 = 0;
                }
            }
        }
    }
}

#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    let window = use_window::<CoreCommand, ()>(&cx);
    let count = use_read(&cx, COUNT);
    let disabled = count.0 == 0;

    cx.render(rsx! {
        h1 { "Counter Example" }
        p { "count: {count.0}" }
        button {
            onclick: move |_| window.send(CoreCommand::Decrement),
            disabled: "{disabled}",
            "-",
        }
        button {
            onclick: move |_| window.send(CoreCommand::Reset),
            disabled: "{disabled}",
            "Reset"
        }
        button {
            onclick: move |_| window.send(CoreCommand::Increment),
            "+",
        }
    })
}