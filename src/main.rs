use gtk::prelude::*;
use std::process::Command;

const PIXEL_SIZE: i32 = 70;
const BUTTON_WIDTH: i32 = 128;
const BUTTON_HEIGHT: i32 = 128;
const SPACING: i32 = 20;

enum ActionType {
    Logout,
    Reboot,
    Lock,
    Shutdown,
    Suspend,
    Hibernate,
}

struct Action {
    command: String,
    button: gtk::Button,
    label: gtk::Label,
}

impl Action {
    fn new(action_type: ActionType, action_name: &str, command: &str, hidden: bool) -> Action {
        let icon_name = match action_type {
            ActionType::Logout => "system-log-out",
            ActionType::Reboot => "system-reboot",
            ActionType::Lock => "system-lock-screen",
            ActionType::Shutdown => "system-shutdown",
            ActionType::Suspend => "system-suspend",
            ActionType::Hibernate => "system-hibernate",
        };

        Action {
            command: command.to_string(),
            button: create_button(create_icon(&icon_name), hidden),
            label: create_label(&action_name, hidden),
        }
    }

    fn button_click(self) {
        self.button.connect_clicked(move |_| {
            Command::new("sh")
                .arg("-c")
                .arg(&self.command)
                .spawn()
                .expect("FAILED TO EXECUTE");
        });
    }
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("Chau")
        .window_position(gtk::WindowPosition::CenterAlways)
        .expand(false)
        .decorated(false)
        .resizable(false)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();

    let grid = gtk::Grid::builder()
        .margin(SPACING)
        .column_spacing(SPACING)
        .row_spacing(SPACING)
        .orientation(gtk::Orientation::Horizontal)
        .valign(gtk::Align::Center)
        .expand(false)
        .halign(gtk::Align::Center)
        .build();

    let logout = Action::new(
        ActionType::Logout,
        "Logout",
        "loginctl terminate-user $USER",
        false,
    );
    let reboot = Action::new(ActionType::Reboot, "Reboot", "systemctl reboot", false);
    let lock = Action::new(ActionType::Lock, "Lock", "swaylock", false);
    let shutdown = Action::new(
        ActionType::Shutdown,
        "Shutdown",
        "systemctl poweroff",
        false,
    );
    let suspend = Action::new(ActionType::Suspend, "Suspend", "systemctl suspend", false);
    let hibernate = Action::new(
        ActionType::Hibernate,
        "Hibernate",
        "systemctl hibernate",
        false,
    );

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

    logout.button_click();
    reboot.button_click();
    lock.button_click();
    shutdown.button_click();
    suspend.button_click();
    hibernate.button_click();

    window.add(&grid);

    window.show_all();
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

fn main() {
    let application = gtk::Application::builder()
        .application_id("com.claaj.chau")
        .build();

    application.connect_startup(build_ui);
    application.run();
}
