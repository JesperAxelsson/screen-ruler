#![allow(dead_code)]
#![allow(unused_imports)]
// use anyhow::Result;
// use chumsky::prelude::*;
// use nom::Parser::pre

use std::{
    thread,
    time::{Duration, Instant, SystemTime},
};

use chrono::Utc;
use parse_xrandr::{Port, parse_output};
use parser::Parser;

mod parser;
mod parse_xrandr;


fn main() {
    let mut ports = get_ports();

    print_ports(&ports);
    handle_ports(&ports, &Vec::new());

    loop {
        let old_ports = ports;
        ports = get_ports();
        handle_ports(&ports, &old_ports);

        thread::sleep(Duration::from_secs(2));
    }
}

fn handle_ports(ports: &Vec<Port>, old_ports: &Vec<Port>) {
    if old_ports != ports {
        // Print ports
        print_ports(&ports);

        let port = ports.iter().find(|p| p.port == "HDMI-0");
        // for port in ports.iter().filter(|p| p.connected) {
        if port.is_some() && port.unwrap().port == "HDMI-0" && port.unwrap().connected {
            if port.unwrap().resolution.is_none() {
                let mut ports = get_ports();
                while let Some(p) = ports.iter().find(|p| p.port == "HDMI-0").cloned() {
                    // Check that we swiched successfully
                    thread::sleep(Duration::from_secs(2));
                    if p.connected && p.resolution.is_some() {
                        break;
                    }

                    println!("Switch to {:?}", port);
                    run_change_to_hdmi_0();

                    thread::sleep(Duration::from_secs(5));
                    ports = get_ports();
                }
            }

            for port in ports.iter() {
                if port.port != "HDMI-0" && port.connected {
                    turn_off_port(&port.port);
                }
            }
        } else {
            thread::sleep(Duration::from_secs(5));

            for port in ports.iter() {
                if port.connected && !port.resolution.is_some() {
                    turn_auto_port(&port.port);
                }
            }
        }

        // if port.port == "DVI-D-0" && port.connected && port.resolution.is_none() {
        //     println!("Switch to {:?}", port);
        //     thread::sleep(Duration::from_secs(5));
        //     run_change_to_dvi();
        // }
        // }
    }
}

fn print_ports(ports: &Vec<Port>) {
    let dt = Utc::now();
    println!("Ports {}", dt.format("%Y-%m-%d %H:%M:%S").to_string());
    for port in ports.iter().filter(|p| p.connected) {
        println!("- {:?}", port);
    }
}

fn run_xrandr() -> String {
    use std::process::Command;
    let output = Command::new("sh")
        .arg("-c")
        .arg("xrandr --props")
        .output()
        .expect("failed to execute xrandr --props");

    return String::from_utf8_lossy(&output.stdout).to_string();
}

fn run_change_to_dvi() {
    use std::process::Command;
    let output = Command::new("sh")
        .arg("-c")
        .arg("xrandr --output DVI-D-0 --mode 2560x1440")
        .output()
        .expect("Failed to read dvi switch output");

    println!("Change to DVI-D: {:?}", output);
}

fn run_change_to_hdmi_0() {
    use std::process::Command;
    let output = Command::new("sh")
        .arg("-c")
        .arg("xrandr --output HDMI-0 --mode 3840x2160")
        .output()
        .expect("Failed to read hdmi switch output");

    println!("Change to HDMI-0: {:?}", output);
}

fn turn_auto_port(port_name: &str) {
    use std::process::Command;
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("xrandr --output {port_name} --auto"))
        .output()
        .expect("Failed to read turn_auto_port");

    println!("Turn auto to {:?}: {:?}", port_name, output);
}

fn turn_off_port(port_name: &str) {
    use std::process::Command;
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("xrandr --output {port_name} --off"))
        .output()
        .expect("Failed to read turn_off_port");

    println!("Turn off to {:?}: {:?}", port_name, output);
}

fn get_ports() -> Vec<Port> {
    let data = run_xrandr();
    parse_output(data)
}

// screen 0: minimum 8 x 8, current 5120 x 1600, maximum 32767 x 32767
// DVI-D-0 connected 2560x1440+2560+0 (normal left inverted right x axis y axis) 597mm x 336mm
//         CTM: 0 1 0 0 0 0 0 0 0 1 0 0 0 0 0 0
//                 0 1
//         CscMatrix: 65536 0 0 0 0 65536 0 0 0 0 65536 0
//         EDID:
//                 00ffffffffffff0010ac55a04c363630
//                 17140103803c2278ea8e05ad4f33b026
//                 0d5054a54b008100b300d100714fa940
//                 818001010101565e00a0a0a029503020
//                 350055502100001a000000ff00473630
//                 36543035533036364c0a000000fc0044
//                 454c4c2055323731310a2020000000fd
//                 0031561d711c000a2020202020200021
//         BorderDimensions: 4
//                 supported: 4
//         Border: 0 0 0 0
//                 range: (0, 65535)
//         SignalFormat: TMDS
//                 supported: TMDS
//         ConnectorType: DVI-D
//         ConnectorNumber: 0
//         _ConnectorLocation: 0
//         non-desktop: 0
//                 supported: 0, 1
//    2560x1440     59.95*+
//    1920x1200     59.88
//    1680x1050     59.95
//    1600x1200     60.00
//    1280x1024     75.02    60.02
//    1280x800      59.81
//    1152x864      75.00
//    1024x768      75.03    60.00
//    800x600       75.00    60.32
//    640x480       75.00    59.94

// #[derive(Clone, Debug)]
// pub enum Port {
//     Screen(String),
//     Connected(String),
//     Disconnected(String),
// }

#[derive(Debug)]
pub struct PortInfo {
    pub port_name: String,
    pub resolutions: Vec<Resolution>,
}

#[derive(Debug)]
pub struct Resolution {
    pub resolution: String,
    pub freq: Vec<String>,
}
