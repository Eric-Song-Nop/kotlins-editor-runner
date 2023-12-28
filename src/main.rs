mod editor;

use glib::clone;
use gtk::{prelude::*, Box, Button, TextView};
use gtk::{Application, ApplicationWindow};

const TMP_FILE: &str = "/tmp/main.kts";

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

    let buffer = editor::HighlightEditor::new();

    // Create a buffer for the output
    let buffer_output = gtk::TextBuffer::new(None::<&gtk::TextTagTable>);
    let buffer_error = gtk::TextBuffer::new(None::<&gtk::TextTagTable>);
    let editor_title = gtk::Label::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .label("Editor")
        .build();
    let output_title = gtk::Label::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .label("Output")
        .build();
    let error_title = gtk::Label::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .label("Error")
        .build();

    let tv_output = TextView::builder()
        .buffer(&buffer_output)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .editable(false)
        .build();


    let tv_error = TextView::builder()
        .buffer(&buffer_error)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .editable(false)
        .build();

    let tv = TextView::builder()
        .buffer(&buffer)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    gtk_box.append(&button);
    gtk_box.append(&editor_title);
    gtk_box.append(&tv);
    gtk_box.append(&output_title);
    gtk_box.append(&tv_output);
    gtk_box.append(&error_title);
    gtk_box.append(&tv_error);

    button.connect_clicked(move |btn| {
        // Save text in the buffer into a tmp file
        let text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false);
        glib::spawn_future_local(clone!(@weak btn, @weak buffer_error, @weak buffer_output => async move {
            btn.set_label("Running ...");
            // Disable the button
            btn.set_sensitive(false);

            std::fs::write(TMP_FILE, text).unwrap();
            btn.set_label("Running...");
            let mut start_iter = buffer_output.start_iter();
            let mut end_iter = buffer_output.end_iter();
            buffer_output.delete(&mut start_iter, &mut end_iter);
            let mut start_iter = buffer_error.start_iter();
            let mut end_iter = buffer_error.end_iter();
            buffer_error.delete(&mut start_iter, &mut end_iter);
            // Run the file with kotlin
            let output = async_std::process::Command::new("kotlinc")
                .arg("-script")
                .arg(TMP_FILE)
                .output()
                .await
                .unwrap();

            // Print the output no matter stdout or stderr
            let stdoutput = String::from_utf8(output.stdout).unwrap();
            let stderror = String::from_utf8(output.stderr).unwrap();
            println!("{}", stdoutput);
            println!("{}", stderror);
            let mut start_iter = buffer_output.start_iter();

            buffer_output.insert(&mut start_iter, &stdoutput);

            let mut start_iter = buffer_error.start_iter();
            buffer_error.insert(&mut start_iter, &stderror);

            // Enable the button
            btn.set_sensitive(true);
            btn.set_label("Run");
        }));
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Kotlin Script Runner")
        .child(&gtk_box)
        .build();

    window.present();
}
