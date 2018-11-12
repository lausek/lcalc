use gtk::prelude::*;

pub struct Cmdline
{
    entry: gtk::Entry,
}

impl Cmdline
{
    pub fn new() -> Self
    {
        Self {
            entry: gtk::Entry::new(),
        }
    }

    pub fn widget(&self) -> &gtk::Entry
    {
        &self.entry
    }

    pub fn buffer(&self) -> gtk::EntryBuffer
    {
        self.entry.get_buffer()
    }
}
