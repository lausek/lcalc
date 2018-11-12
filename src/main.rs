#![feature(custom_attribute)]

extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
extern crate treecalc;

mod app;
mod widget;

use relm::Widget;

fn main()
{
    if let Err(e) = app::App::run(()) {
        println!("{:?}", e);
    }
}
