use pugl_rs::{Event, OpenGl, World};

fn main() {
    let mut world = World::new_program().unwrap();
    let view = world
        .new_view(OpenGl {
            ..Default::default()
        })
        .with_resizable(false)
        .with_size(200, 200)
        .with_event_handler(|view, event| {
            if matches!(event, Event::Close) {
                std::process::exit(0);
            }

            if matches!(event, Event::Update) {
                view.obscure_view();
            }

            if let Event::Expose { backend, .. } = &event {
                unsafe {
                    let gl_clear_color: fn(f32, f32, f32, f32) =
                        std::mem::transmute(backend.get_proc_address(c"glClearColor"));
                    let gl_clear: fn(u32) =
                        std::mem::transmute(backend.get_proc_address(c"glClear"));

                    gl_clear_color(1.0, 1.0, 0.0, 1.0);
                    gl_clear(0x4000);
                }
            }

            println!("{:?} {:?}", event, view);
        })
        .realize()
        .unwrap();

    view.show_aggressive();

    loop {
        let _ = world.update(None);
    }
}
