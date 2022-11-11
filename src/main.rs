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
use parse_xrandr::{parse_output, Port};
use parser::Parser;
use xrandr::{run_change_to_hdmi_0, run_xrandr, turn_auto_port, turn_off_port};

mod parse_xrandr;
mod parser;
mod xrandr;

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

fn get_ports() -> Vec<Port> {
    let data = run_xrandr();
    parse_output(data)
}
