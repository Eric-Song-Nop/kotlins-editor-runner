mod editor;
use gtk::{prelude::*, Box, Button, TextView};
use gtk::{Application, ApplicationWindow};

const APP_ID: &str = "org.eric.KotlinsRunner";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let gtk_box = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let button = Button::builder()
        .label("Run")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(|btn| {
        btn.set_label("Hello world");
    });

    let buffer = editor::HighlightEditor::new();

    let tv = TextView::builder()
        .buffer(&buffer)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    gtk_box.append(&button);
    gtk_box.append(&tv);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Kotlin Script Runner")
        .child(&gtk_box)
        .build();

    window.present();
}
