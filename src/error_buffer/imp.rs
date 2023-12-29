
use gtk::glib;
use gtk::prelude::{TextBufferExt, TextBufferExtManual};
use gtk::subclass::prelude::*;

#[derive(Default)]
pub struct ErrorBuffer {}

#[glib::object_subclass]
impl ObjectSubclass for ErrorBuffer {
    const NAME: &'static str = "ErrorBuffer";
    type Type = super::ErrorBuffer;
    type ParentType = gtk::TextBuffer;

    fn new() -> Self {
        Self {}
    }
}

impl ObjectImpl for ErrorBuffer {
    fn constructed(&self) {
        // Create a tag for error messages
        self.obj()
            .create_tag(Some("ErrorMsg"), &[("foreground", &"red".to_string())]);
    }
}

impl TextBufferImpl for ErrorBuffer {
    fn changed(&self) {
        self.parent_changed();
        // Remove all tags
        let start_iter = self.obj().start_iter();
        let end_iter = self.obj().end_iter();
        self.obj().remove_all_tags(&start_iter, &end_iter);

        // Highlight error messages
        // iterate over all lines
        let mut start_iter = self.obj().start_iter();
        let mut end_iter = self.obj().start_iter();
        while !end_iter.is_end() {
            end_iter.forward_to_line_end();
            // check it line starts with "/tmp/main.kts:([0-9]+):([0-9]+):error:"
            let line = self.obj().text(&start_iter, &end_iter, false);
            if line.starts_with(r"/tmp/main.kts:") {
                // Do the regex search
                let re = regex::Regex::new(r"/tmp/main\.kts:(\d+):(\d+): error:").unwrap();
                let caps = re.captures(&line);
                if let Some(caps) = caps {
                    let line_number = caps.get(1).unwrap().as_str();
                    let column_number = caps.get(2).unwrap().as_str();

                    println!("Found error on line {} column {}", line_number, column_number);

                    // get length of captured string
                    let length = caps.get(0).unwrap().as_str().len();

                    // set the tag
                    self.obj().apply_tag_by_name(
                        "ErrorMsg",
                        &start_iter,
                        &self.obj().iter_at_offset(start_iter.offset() + length as i32),
                    );
                }
            }
            start_iter.forward_line();
            end_iter.forward_line();
        }
    }
}
