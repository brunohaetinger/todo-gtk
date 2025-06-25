use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, Entry, ListBox, Label, Orientation, Box as GtkBox};

fn main() {
    let app = Application::builder()
        .application_id("com.example.TodoApp")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("To-Do List")
        .default_width(300)
        .default_height(400)
        .build();

    let vbox = GtkBox::new(Orientation::Vertical, 6);
    window.set_child(Some(&vbox));

    let entry = Entry::builder()
        .placeholder_text("Enter a new task")
        .build();

    let add_button = Button::with_label("Add");

    let listbox = ListBox::new();

    // Clone to move into the button callback
    let entry_clone = entry.clone();
    let listbox_clone = listbox.clone();

    add_button.connect_clicked(move |_| {
        let text = entry_clone.text().to_string();
        if !text.is_empty() {
            let label = Label::new(Some(&text));
            listbox_clone.append(&label);
            entry_clone.set_text("");
        }
    });

    vbox.append(&entry);
    vbox.append(&add_button);
    vbox.append(&listbox);

    window.show();
}

