// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use pnet::datalink;

use log::info;

use std::io;


fn main() {
    env_logger::init();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_interfaces,print_selected_interface])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command(rename_all = "snake_case")]
fn get_interfaces() -> Vec<String> {
    
    let interfaces = datalink::interfaces();
    println!("Fetching network interfaces");

    let names: Vec<String> = interfaces.iter().map(|iface| {
        let name = iface.name.clone();
        println!("Found interface: {}", name);
        name
    }).collect();

    names
}

#[tauri::command(rename_all = "snake_case")]
fn print_selected_interface(interface_name: String) {
    println!("You have selected the interface: {}", interface_name);
}


