use gtk::{
    glib, prelude::*, Application, ApplicationWindow, Box, Button, Image, Label, Orientation,
};

const APP_ID: &str = "com.CenTdemeern1.buttonrepro";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let image = Image::builder()
        .file("test.svg")
        .vexpand(true) // Seems to have no effect...
        .build();
    let label = Label::builder().label("label").build();
    let vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .valign(gtk::Align::Center) // ...with this enabled
        .build();
    // When using vexpand on the image, it pushes the label down, I won't want that
    // I want the image to grow as big as it can while staying square and centered together with the label
    vbox.append(&image);
    vbox.append(&label);

    let button = Button::builder().child(&vbox).build();

    button.connect_clicked(|_| println!("Button clicked!"));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Button issue repro")
        .child(&button)
        .build();

    window.set_default_size(400, 600); // Feel free to comment this out

    // Present window
    window.present();
}
