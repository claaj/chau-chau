use gtk::prelude::*;
use std::process::Command;

const PIXEL_SIZE: i32 = 70;
const BUTTON_WIDTH: i32 = 128;
const BUTTON_HEIGHT: i32 = 128;
const LOGOUT_ICON_NAME: &str = "system-log-out";
const REBOOT_ICON_NAME: &str = "system-reboot";
const LOCK_ICON_NAME: &str = "system-lock-screen";
const SHUTDOWN_ICON_NAME: &str = "system-shutdown";
const SUSPEND_ICON_NAME: &str = "system-suspend";
const HIBERNATE_ICON_NAME: &str = "system-hibernate";

struct Action {
    command: String,
    button: gtk::Button,
    label: gtk::Label,
}

impl Action {
    fn new(action_name: &str, icon_name: &str, command: &str, hidden: bool) -> Action {
        Action {
            command: command.to_string(),
            button: create_button(create_icon(&icon_name), hidden),
            label: create_label(&action_name, hidden),
        }
    }
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("Power Menu")
        .window_position(gtk::WindowPosition::CenterAlways)
        .expand(false)
        .decorated(false)
        .resizable(false)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();

    let grid = gtk::Grid::builder()
        .column_homogeneous(true)
        .margin(20)
        .column_spacing(20)
        .row_spacing(20)
        .orientation(gtk::Orientation::Horizontal)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .expand(false)
        .build();

    let logout = Action::new("Logout", LOGOUT_ICON_NAME, "echo logout", false);
    let reboot = Action::new("Reboot", REBOOT_ICON_NAME, "echo reboot", false);
    let lock = Action::new("Lock", LOCK_ICON_NAME, "echo lock", false);
    let shutdown = Action::new("Shutdown", SHUTDOWN_ICON_NAME, "shutdown now", false);
    let suspend = Action::new("Suspend", SUSPEND_ICON_NAME, "echo suspend", false);
    let hibernate = Action::new("Hibernate", HIBERNATE_ICON_NAME, "echo hibernate", false);

    grid.attach(&logout.button, 0, 0, 1, 1);
    grid.attach(&reboot.button, 1, 0, 1, 1);
    grid.attach(&lock.button, 2, 0, 1, 1);
    grid.attach(&shutdown.button, 3, 0, 1, 1);
    grid.attach(&suspend.button, 4, 0, 1, 1);
    grid.attach(&hibernate.button, 5, 0, 1, 1);

    grid.attach(&logout.label, 0, 1, 1, 1);
    grid.attach(&reboot.label, 1, 1, 1, 1);
    grid.attach(&lock.label, 2, 1, 1, 1);
    grid.attach(&shutdown.label, 3, 1, 1, 1);
    grid.attach(&suspend.label, 4, 1, 1, 1);
    grid.attach(&hibernate.label, 5, 1, 1, 1);

    execute_command(&logout.button, logout.command);
    execute_command(&reboot.button, reboot.command);
    execute_command(&lock.button, lock.command);
    execute_command(&shutdown.button, shutdown.command);
    execute_command(&suspend.button, suspend.command);
    execute_command(&hibernate.button, hibernate.command);

    window.add(&grid);

    window.show_all();
}

fn main() {
    let application = gtk::Application::builder()
        .application_id("com.claaj.powermenu")
        .build();

    application.connect_activate(build_ui);
    application.run();
}

fn create_icon(icon_name: &str) -> gtk::Image {
    gtk::Image::builder()
        .icon_name(icon_name)
        .pixel_size(PIXEL_SIZE)
        .build()
}

fn create_button(icon: gtk::Image, hidden: bool) -> gtk::Button {
    gtk::Button::builder()
        .no_show_all(hidden)
        .width_request(BUTTON_WIDTH)
        .height_request(BUTTON_HEIGHT)
        .image(&icon)
        .build()
}

fn create_label(action_name: &str, hidden: bool) -> gtk::Label {
    gtk::Label::builder()
        .no_show_all(hidden)
        .label(action_name)
        .justify(gtk::Justification::Center)
        .selectable(false)
        .build()
}

fn execute_command(button: &gtk::Button, command: String) {
    button.connect_clicked(move |_| {
        Command::new("sh")
            .arg("-c")
            .arg(&command)
            .spawn()
            .expect("FAILED TO EXECUTE");
    });
}
