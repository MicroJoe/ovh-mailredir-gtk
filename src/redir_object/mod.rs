mod imp;

glib::wrapper! {
    pub struct RedirObject(ObjectSubclass<imp::RedirObject>);
}

impl RedirObject {
    pub fn new(src: String) -> Self {
        glib::Object::new(&[("src", &src)]).expect("Failed to create `RedirObject`.")
    }
}
