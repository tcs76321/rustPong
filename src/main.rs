use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main() {
    let app = Application::builder().application_id("rustPong").build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(240)
            .default_height(240)
            .title("Pong in Rust")
            .build();
        window.show_all();
    });

    app.run();
}
