use pugl_rs::{Event, World};

fn main() {
    let mut world = World::new_program().unwrap();
    let view = world
        .new_view(())
        .with_resizable(false)
        .with_size(200, 200)
        .with_event_handler(|view, event| {
            if matches!(event, Event::Close) {
                std::process::exit(0);
            }

            if matches!(event, Event::Update) {
                view.obscure_view();
            }

            println!("{:?}", event);
        })
        .realize()
        .unwrap();

    view.show_aggressive();

    loop {
        let _ = world.update(None);
    }
}
