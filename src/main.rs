use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

const APP_ID: &str = "chrsin.multibar";

//Example bar written in python: https://gist.github.com/johnlane/351adff97df196add08a
//gtk-rs documentation: https://gtk-rs.org/gtk4-rs/stable/latest/book/hello_world.html
//rust book: https://rust-book.cs.brown.edu/ch02-00-guessing-game-tutorial.html

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_bar);
    
    app.run()
}

fn build_bar(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("multibar")
        .build();

    let monitors = window.display().monitors();


    window.set_decorated(false);
    window.present();
}
