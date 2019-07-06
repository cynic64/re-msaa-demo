extern crate render_engine;

use render_engine as re;

use re::app::App;
use re::exposed_tools::*;

fn main() {
    println!("This is the MSAA demo. Press E to enable 4x MSAA, press D to disable antialiasing.");

    let mut app = App::new();

    app.new_vbuf_from_verts(
        &[
            Vertex {
                position: [0.0, -1.0],
                color: [1.0, 0.0, 0.0, 1.0],
            },
            Vertex {
                position: [-1.0, 1.0],
                color: [0.0, 1.0, 0.0, 1.0],
            },
            Vertex {
                position: [1.0, 1.0],
                color: [0.0, 0.0, 1.0, 1.0],
            },
        ]);

    loop {
        {
            let mut enable = false;
            let mut disable = false;

            app.unprocessed_events
                .iter()
                .for_each(|&keycode| match keycode {
                    VirtualKeyCode::E => enable = true,
                    VirtualKeyCode::D => disable = true,
                    _ => {}
                });

            if enable { app.enable_multisampling(); }
            if disable { app.disable_multisampling(); }
        }

        app.draw_frame();

        if app.done {
            break;
        }
    }

    app.print_fps();
}
