mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct ErrorBuffer(ObjectSubclass<imp::ErrorBuffer>)
        @extends gtk::TextBuffer;
}

impl ErrorBuffer {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for ErrorBuffer {
    fn default() -> Self {
        Self::new()
    }
}
