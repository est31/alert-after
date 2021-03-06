#[cfg(target_os = "macos")]
extern crate mac_notification_sys;
extern crate notify_rust;

use std::{env, io, process};
use std::io::Write;

#[cfg(target_os = "macos")]
fn notify(msg: &str) {
    let bundle = mac_notification_sys::get_bundle_identifier("safari").unwrap();

    mac_notification_sys::set_application(&bundle).unwrap();
    mac_notification_sys::send_notification(msg, &None, "finished executing", &None).unwrap();
}

#[cfg(not(target_os = "macos"))]
fn notify(msg: &str) {
    use notify_rust::Notification;
    Notification::new()
        .summary(msg)
        .body("finished executing")
        .show()
        .unwrap();
}

fn main() {
    let mut args = env::args();

    let _ = args.next().unwrap();

    let program_name = match args.next() {
        Some(program_name) => program_name,
        None => {
            writeln!(io::stderr(), "usage: aa <program name and args>")
                .expect("could not write to stderr");
            process::exit(1);
        }
    };

    let mut command = process::Command::new(program_name.clone());
    let args = args.collect::<Vec<_>>();
    command.args(args.clone());

    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(e) => {
            writeln!(io::stderr(),
                     "aa: Unknown command '{}': {}",
                     program_name,
                     e)
                    .expect("could not write to stderr");
            process::exit(1);
        }
    };

    let exit_status = child.wait().expect("failed to wait on command");


    let mut full_cmd = program_name;
    full_cmd.push_str(" ");
    full_cmd.push_str(&args.join(" "));

    notify(&full_cmd);

    if let Some(code) = exit_status.code() {
        process::exit(code);
    }
}
