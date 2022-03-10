use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Label, ListView, PolicyType, ScrolledWindow,
    SignalListItemFactory, SingleSelection,
};

mod redir_object;

use redir_object::RedirObject;

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("com.github.microjoe.ovh-mailredir-gtk")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a `Vec<IntegerObject>` with numbers from 0 to 100_000
    let mut vector: Vec<RedirObject> = Vec::new();
    for _ in 1..100 {
        vector.push(RedirObject::new("Hello".into()));
        vector.push(RedirObject::new("World".into()));
    }

    // Create new model
    let model = gio::ListStore::new(RedirObject::static_type());

    // Add the vector to the model at position 0 and remove 0 elements
    model.splice(0, 0, &vector);

    let factory = SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        let label = Label::new(None);
        list_item.set_child(Some(&label));
    });

    factory.connect_bind(move |_, list_item| {
        // Get `IntegerObject` from `ListItem`
        let integer_object = list_item
            .item()
            .expect("The item has to exist.")
            .downcast::<RedirObject>()
            .expect("The item has to be an `IntegerObject`.");

        // Get `i32` from `IntegerObject`
        let src = integer_object.property::<String>("src");

        // Get `Label` from `ListItem`
        let label = list_item
            .child()
            .expect("The child has to exist.")
            .downcast::<Label>()
            .expect("The child has to be a `Label`.");

        // Set "label" to "number"
        label.set_label(&src);
    });

    let selection_model = SingleSelection::new(Some(&model));
    let list_view = ListView::new(Some(&selection_model), Some(&factory));

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&list_view)
        .build();

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .default_width(600)
        .default_height(300)
        .child(&scrolled_window)
        .build();

    // Present window
    window.present();
}
