extern crate gtk;

#[macro_use]
extern crate relm;

use gtk::prelude::*;

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
impl relm::Widget for Win {
    fn model() -> Model {
        Model { count: 0 }
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
                        label: "Decrease",
                    },

                    #[name="inc_button"]
                    gtk::Button {
                        clicked => Msg::Increase,
                        label: "Increase",
                    },
                }
            },

            delete_event(_, _) => (Msg::Quit, gtk::Inhibit(false)),
            title: &create_window_title(self.model.count),
        }
    }
}

fn create_window_title(count: i32) -> std::string::String {
    format!("Counter: {}", count)
}

fn main() {
    relm::run::<Win>(()).unwrap();
}
