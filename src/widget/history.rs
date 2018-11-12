use gtk::prelude::*;

pub struct History
{
    text_view: gtk::TextView,
}

impl History
{
    pub fn new() -> Self
    {
        Self {
            text_view: gtk::TextView::new(),
        }
    }

    pub fn push(&mut self, mut msg: String)
    {
        if let Some(buffer) = self.text_view.get_buffer() {
            msg.push('\n');
            buffer.insert(&mut buffer.get_end_iter(), msg.as_ref());
        }
    }

    pub fn widget(&self) -> &gtk::TextView
    {
        &self.text_view
    }
}
