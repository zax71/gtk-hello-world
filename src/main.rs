use gtk::{ glib, Application, ApplicationWindow, Box, Label };
use gtk::prelude::*;

const APP_ID: &str = "me.zax71.HelloWorld";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let todo1 = Label::builder().label("A todo 1").build();
    let todo2 = Label::builder().label("A todo 2").build();

    let todos_list = Box::new(gtk::Orientation::Horizontal, 10);
    todos_list.append(&todo1);
    todos_list.append(&todo2);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hello GTK")
        .child(&todos_list)
        .build();

    window.present();
}
