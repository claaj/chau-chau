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
    Poweroff,
    Suspend,
    Hibernate,
}

struct Action {
    command: &'static str,
    shortcut: &'static str,
    button: gtk::Button,
    label: gtk::Label,
    hidden: bool,
}

impl Action {
    fn new(
        action_type: ActionType,
        name: &str,
        command: &'static str,
        shortcut: &'static str,
        hidden: bool,
    ) -> Action {
        let icon_name = match action_type {
            ActionType::Logout => "system-log-out",
            ActionType::Reboot => "system-reboot",
            ActionType::Lock => "system-lock-screen",
            ActionType::Poweroff => "system-shutdown",
            ActionType::Suspend => "system-suspend",
            ActionType::Hibernate => "system-hibernate",
        };

        let icon = gtk::Image::builder()
            .icon_name(icon_name)
            .pixel_size(PIXEL_SIZE)
            .build();

        let label = gtk::Label::builder()
            .no_show_all(hidden)
            .label(name)
            .justify(gtk::Justification::Center)
            .selectable(false)
            .build();

        let button = gtk::Button::builder()
            .no_show_all(hidden)
            .width_request(BUTTON_WIDTH)
            .height_request(BUTTON_HEIGHT)
            .image(&icon)
            .build();
        Action {
            command,
            shortcut,
            button,
            label,
            hidden,
        }
    }

    fn button_click(&self) {
        let action_command = self.command.clone();
        self.button
            .to_owned()
            .connect_clicked(move |_| execute_command(&action_command));
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
        // "echo logout",
        "loginctl terminate-user $USER",
        "q",
        false,
    );
    let reboot = Action::new(
        ActionType::Reboot,
        "Reboot",
        // "echo reboot",
        "systemctl reboot",
        "r",
        false,
    );
    let lock = Action::new(
        ActionType::Lock,
        "Lock",
        // "echo lock",
        "swaylock",
        "l",
        false,
    );
    let poweroff = Action::new(
        ActionType::Poweroff,
        "Power off",
        // "echo poweroff",
        "systemctl poweroff",
        "p",
        false,
    );
    let suspend = Action::new(
        ActionType::Suspend,
        "Suspend",
        // "echo suspend",
        "systemctl suspend",
        "s",
        false,
    );
    let hibernate = Action::new(
        ActionType::Hibernate,
        "Hibernate",
        // "echo hibernate",
        "systemctl hibernate",
        "h",
        false,
    );

    grid.attach(&logout.button, 0, 0, 1, 1);
    grid.attach(&reboot.button, 1, 0, 1, 1);
    grid.attach(&lock.button, 2, 0, 1, 1);
    grid.attach(&poweroff.button, 3, 0, 1, 1);
    grid.attach(&suspend.button, 4, 0, 1, 1);
    grid.attach(&hibernate.button, 5, 0, 1, 1);

    grid.attach(&logout.label, 0, 1, 1, 1);
    grid.attach(&reboot.label, 1, 1, 1, 1);
    grid.attach(&lock.label, 2, 1, 1, 1);
    grid.attach(&poweroff.label, 3, 1, 1, 1);
    grid.attach(&suspend.label, 4, 1, 1, 1);
    grid.attach(&hibernate.label, 5, 1, 1, 1);

    window.add(&grid);

    window.connect_key_release_event(move |window, key| {
        let key_pressed = key.keyval().name().unwrap();
        let key_str = key_pressed.as_str();
        //Match expressions doesn't work with custom shortcuts
        if key_str == "Escape" {
            window.close();
        } else if !logout.hidden && key_str.eq(logout.shortcut) {
            execute_command(&logout.command);
        } else if !reboot.hidden && key_str.eq(reboot.shortcut) {
            execute_command(&reboot.command);
        } else if !lock.hidden && key_str.eq(lock.shortcut) {
            execute_command(&lock.command);
        } else if !poweroff.hidden && key_str.eq(poweroff.shortcut) {
            execute_command(&poweroff.command);
        } else if !suspend.hidden && key_str.eq(suspend.shortcut) {
            execute_command(&suspend.command);
        } else if !hibernate.hidden && key_str.eq(hibernate.shortcut) {
            execute_command(&hibernate.command);
        }
        Inhibit(false)
    });

    logout.button_click();
    reboot.button_click();
    lock.button_click();
    poweroff.button_click();
    suspend.button_click();
    hibernate.button_click();

    window.show_all();
}

fn execute_command(command: &str) {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("FAILED TO EXECUTE");
}

fn main() {
    let application = gtk::Application::builder()
        .application_id("com.github.claaj.chau")
        .build();

    application.connect_activate(|app| {
        build_ui(app);
    });
    application.run();
}
