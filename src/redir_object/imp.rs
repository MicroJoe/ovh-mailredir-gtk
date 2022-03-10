use glib::{ParamFlags, ParamSpec, ParamSpecString, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;

use std::cell::RefCell;

// Object holding the state
#[derive(Default)]
pub struct RedirObject {
    src: RefCell<Option<String>>,
}

#[glib::object_subclass]
impl ObjectSubclass for RedirObject {
    const NAME: &'static str = "RedirObject";
    type Type = super::RedirObject;
}

// Trait shared by all GObjects
impl ObjectImpl for RedirObject {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpecString::new(
                // Name
                "src",
                // Nickname
                "src",
                // Short description
                "src",
                // Default value
                None,
                // The property can be read and written to
                ParamFlags::READWRITE,
            )]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "src" => {
                let input = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.src.replace(input);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "src" => self.src.borrow().to_value(),
            _ => unimplemented!(),
        }
    }
}
