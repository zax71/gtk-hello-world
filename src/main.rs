use gtk::prelude::*;
use gtk::{ glib, Application, ApplicationWindow, Button };

const APP_ID: &str = "me.zax71.HelloWorld";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Click me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_end(12)
        .margin_start(12)
        .build();

    button.connect_clicked(|button| {
        if button.label().unwrap() == "Click me!" {
            button.set_label("Clicked");
        } else {
            button.set_label("Click me!");
        }
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hello GTK")
        .child(&button)
        .build();

    window.present();
}
