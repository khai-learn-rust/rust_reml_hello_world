extern crate gtk;

#[macro_use]
extern crate relm;

use gtk::prelude::*;
use relm::Widget;

pub struct Model {}

#[derive(relm_derive::Msg)]
pub enum Msg {
    Quit,
}

#[relm_attributes::widget]
impl Widget for Win {
    fn model() -> Model {
        Model {}
    }

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Quit => gtk::main_quit()
        }
    }

    view! {
        gtk::Window {
            gtk::Box {},

            delete_event(_, _) => (Msg::Quit, gtk::Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
