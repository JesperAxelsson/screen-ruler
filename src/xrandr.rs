

pub fn run_xrandr() -> String {
    use std::process::Command;
    let output = Command::new("sh")
        .arg("-c")
        .arg("xrandr --props")
        .output()
        .expect("failed to execute xrandr --props");

    return String::from_utf8_lossy(&output.stdout).to_string();
}

pub fn run_change_to_dvi() {
    use std::process::Command;
    let output = Command::new("sh")
        .arg("-c")
        .arg("xrandr --output DVI-D-0 --mode 2560x1440")
        .output()
        .expect("Failed to read dvi switch output");

    println!("Change to DVI-D: {:?}", output);
}

pub fn run_change_to_hdmi_0() {
    use std::process::Command;
    let output = Command::new("sh")
        .arg("-c")
        .arg("xrandr --output HDMI-0 --mode 3840x2160")
        .output()
        .expect("Failed to read hdmi switch output");

    println!("Change to HDMI-0: {:?}", output);
}

pub fn turn_auto_port(port_name: &str) {
    use std::process::Command;
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("xrandr --output {port_name} --auto"))
        .output()
        .expect("Failed to read turn_auto_port");

    println!("Turn auto to {:?}: {:?}", port_name, output);
}

pub fn turn_off_port(port_name: &str) {
    use std::process::Command;
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("xrandr --output {port_name} --off"))
        .output()
        .expect("Failed to read turn_off_port");

    println!("Turn off to {:?}: {:?}", port_name, output);
}
