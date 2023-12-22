use std::collections::HashMap;

use gtk::glib;
use gtk::prelude::{TextBufferExt, TextBufferExtManual};
use gtk::subclass::prelude::*;

#[derive(Default)]
pub struct HighlightEditor {
    keywords_highlight_table: HashMap<String, String>,
}

#[glib::object_subclass]
impl ObjectSubclass for HighlightEditor {
    const NAME: &'static str = "HighlightEditor";
    type Type = super::HighlightEditor;
    type ParentType = gtk::TextBuffer;

    fn new() -> Self {
        let mut keywords_highlight_table = HashMap::new();
        keywords_highlight_table.insert("fun".to_string(), "blue".to_string());
        keywords_highlight_table.insert("for".to_string(), "blue".to_string());
        keywords_highlight_table.insert("if".to_string(), "blue".to_string());
        keywords_highlight_table.insert("true".to_string(), "green".to_string());
        keywords_highlight_table.insert("false".to_string(), "red".to_string());
        Self {
            keywords_highlight_table,
        }
    }
}

impl ObjectImpl for HighlightEditor {
    fn constructed(&self) {
        // Create a tag for each keyword
        for (keyword, color) in &self.keywords_highlight_table {
            self.obj()
                .create_tag(Some(keyword), &[("foreground", color)]);
        }
    }
}

impl TextBufferImpl for HighlightEditor {
    fn changed(&self) {
        self.parent_changed();
        // Highlight keywords
        for (keyword, _) in &self.keywords_highlight_table {
            let mut start_iter = self.obj().start_iter();

            // println!("Searching for keyword: {}", keyword);

            while let Some((l_iter, r_iter)) = start_iter.forward_search(
                keyword,
                gtk::TextSearchFlags::VISIBLE_ONLY,
                None,
            ) {
                self.obj()
                    .apply_tag_by_name(keyword, &l_iter, &r_iter);
                // println!("Found keyword: {} at {}", keyword, l_iter.offset());
                start_iter = r_iter.clone();
            }
        }
    }
}
