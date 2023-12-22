mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct HighlightEditor(ObjectSubclass<imp::HighlightEditor>)
        @extends gtk::TextBuffer;
}

impl HighlightEditor {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for HighlightEditor {
    fn default() -> Self {
        Self::new()
    }
}
