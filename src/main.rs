use gtk::prelude::*;
use gtk::{ glib, Application, ApplicationWindow };

const APP_ID: &str = "me.zax71.HelloWorld";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder().application(app).title("Hello GTK").build();

    window.present();
}
