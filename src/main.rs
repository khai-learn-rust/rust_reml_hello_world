extern crate gtk;

#[macro_use]
extern crate relm;

use gtk::prelude::*;
use relm::Widget;

pub struct Model {
    count: i32,
}

#[derive(relm_derive::Msg)]
pub enum Msg {
    Decrease,
    Increase,
    Quit,
}

#[relm_attributes::widget]
impl Widget for Win {
    fn model() -> Model {
        Model {
            count: 0,
        }
    }

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Decrease => self.model.count -= 1,
            Msg::Increase => self.model.count += 1,
            Msg::Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: gtk::Orientation::Vertical,

                #[name="label"]
                gtk::Label {
                    text: &self.model.count.to_string()
                },

                #[name="button_container"]
                gtk::Box {
                    orientation: gtk::Orientation::Horizontal,

                    #[name="dec_button"]
                    gtk::Button {
                        clicked => Msg::Decrease,
                        label: "-",
                    },

                    #[name="inc_button"]
                    gtk::Button {
                        clicked => Msg::Increase,
                        label: "+",
                    },
                }
            },

            delete_event(_, _) => (Msg::Quit, gtk::Inhibit(false)),
            title: "Simple Counter",
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
