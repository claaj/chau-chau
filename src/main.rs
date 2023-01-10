use gtk::prelude::*;
use std::process::Command;

const PIXEL_SIZE: i32 = 70;
const BUTTON_WIDTH: i32 = 128;
const BUTTON_HEIGHT: i32 = 128;

struct Action {
    action_name: String,
    icon_name: String,
    command: String,
    hide: bool,
}

impl Action {
    fn create_icon(&self) -> gtk::Image {
        let image = gtk::Image::builder()
            .icon_name(&self.icon_name)
            .pixel_size(PIXEL_SIZE)
            .build();
        image
    }

    fn create_button(&self) -> gtk::Button {
        let button = gtk::Button::builder()
            .no_show_all(self.hide)
            .width_request(BUTTON_WIDTH)
            .height_request(BUTTON_HEIGHT)
            .image(&self.create_icon())
            .build();
        button
    }

    fn create_label(&self) -> gtk::Label {
        let label = gtk::Label::builder()
            .label(&self.action_name)
            .justify(gtk::Justification::Center)
            .selectable(false)
            .build();
        label
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

    let logout = Action {
        action_name: "Logout".to_string(),
        icon_name: "system-log-out".to_string(),
        command: "echo logout command".to_string(),
        hide: false,
    };

    let reboot = Action {
        action_name: "Reboot".to_string(),
        icon_name: "system-reboot".to_string(),
        command: "echo reboot command".to_string(),
        hide: false,
    };

    let lock = Action {
        action_name: "Lock".to_string(),
        icon_name: "system-lock-screen".to_string(),
        command: "echo lock command".to_string(),
        hide: false,
    };

    let shutdown = Action {
        action_name: "Shutdown".to_string(),
        icon_name: "system-shutdown".to_string(),
        command: "echo shutdown command".to_string(),
        hide: false,
    };

    let suspend = Action {
        action_name: "Suspend".to_string(),
        icon_name: "system-suspend".to_string(),
        command: "echo suspend command".to_string(),
        hide: false,
    };

    let hibernate = Action {
        action_name: "Hibernate".to_string(),
        icon_name: "system-hibernate".to_string(),
        command: "echo hibernate command".to_string(),
        hide: false,
    };

    let logout_button = logout.create_button();
    let reboot_button = reboot.create_button();
    let lock_button = lock.create_button();
    let shutdown_button = shutdown.create_button();
    let suspend_button = suspend.create_button();
    let hibernate_button = hibernate.create_button();

    let logout_label = logout.create_label();
    let reboot_label = reboot.create_label();
    let lock_label = lock.create_label();
    let shutdown_label = shutdown.create_label();
    let suspend_label = suspend.create_label();
    let hibernate_label = hibernate.create_label();

    execute_command(&logout_button, logout.command);
    execute_command(&reboot_button, reboot.command);
    execute_command(&lock_button, lock.command);
    execute_command(&shutdown_button, shutdown.command);
    execute_command(&suspend_button, suspend.command);
    execute_command(&hibernate_button, hibernate.command);

    grid.attach(&logout_button, 0, 0, 1, 1);
    grid.attach(&reboot_button, 1, 0, 1, 1);
    grid.attach(&lock_button, 2, 0, 1, 1);
    grid.attach(&shutdown_button, 3, 0, 1, 1);
    grid.attach(&suspend_button, 4, 0, 1, 1);
    grid.attach(&hibernate_button, 5, 0, 1, 1);

    grid.attach(&logout_label, 0, 1, 1, 1);
    grid.attach(&reboot_label, 1, 1, 1, 1);
    grid.attach(&lock_label, 2, 1, 1, 1);
    grid.attach(&shutdown_label, 3, 1, 1, 1);
    grid.attach(&suspend_label, 4, 1, 1, 1);
    grid.attach(&hibernate_label, 5, 1, 1, 1);

    window.add(&grid);

    window.show_all();
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

fn main() {
    let application = gtk::Application::builder()
        .application_id("com.claaj.powermenu")
        .build();

    application.connect_activate(build_ui);
    application.run();
}
