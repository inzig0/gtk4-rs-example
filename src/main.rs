//Makes our life easier since we aren't using gtk3 and gtk4 at the same time
use gtk4 as gtk;
//We need to include gtk::glib so it doesn't conflict with glib
use gtk::glib;
use glib::clone;
use gtk::prelude::*;
//Gives us access to the Orientation enum
use gtk::Orientation::*;

//This is NOT the main function, jump to the bottom for the main function
fn init_window(app: &gtk::Application) {
    //Create a window object belonging to the app
    let window = gtk::ApplicationWindow::new(app);
    //Create a few buttons and things
    let button = gtk::Button::with_label("Hello :3");
    let button2 = gtk::Button::with_label("Goodbye");
    //The window object cannot have multiple children, but this box can
    //Also, you cannot name this object "box", otherwise Rust will expect a pattern
    let gtkbox = gtk::Box::new(Horizontal, 5);

    //Importing outside variables keeps this code from erroring, not entirely sure why
    button.connect_clicked(|_| println!("Hey there!"));
    //This one took a while for me to figure out, so I'll explain it
    //window.close() references a variable that will outlive this function,
    //so we use move to move it into this function. However, the same window object
    //gets referenced outside of this function, so we have to clone the object
    //so that the object exists outside the scope of this function as well as
    //inside the scope of this function. Does that make sense?
    button2.connect_clicked(clone!(@weak window => move |_| {
        println!("Goodbye");
        window.close();
    }));
    //window object adopts our box
    window.set_child(Some(&gtkbox));
    //box adopts both our buttons
    gtkbox.append(&button);
    gtkbox.append(&button2);
    //Make the window visible
    window.present();
}

fn main() {
    //Create the app object, this gives a place for our window(s) to live
    let app = gtk::Application::builder()
        .application_id("com.indigo.gtk4-rs.example")
        .build();
    //When the app is activated(run), go to the init_window() function
    app.connect_activate(init_window);
    //Run the app!
    app.run();
}
