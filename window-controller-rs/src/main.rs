use std::process::Command;

use wmctrl;

fn check_if_wmctrl_exists() -> bool {
    let output = Command::new("which")
        .arg("wmctrl")
        .output()
        .expect("Failed to execute wmctrl");

    if output.status.success() {
        return true;
    }

    return false;
}

fn main() {
    if !check_if_wmctrl_exists() {
        println!("wmctrl is not installed");
        return;
    }

    let windows = wmctrl::get_windows();
    println!("{:#?}", windows);

    // let firefox = wmctrl::utils::find_window_by_title(&windows, "Firefox").unwrap();
    // println!("{}", firefox);
}
