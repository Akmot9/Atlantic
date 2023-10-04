// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use pnet::datalink;

use log::info;

use std::io;


fn main() {
    env_logger::init();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![choose_interface,get_interfaces])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn choose_interface() -> Option<String> {
    let interfaces = datalink::interfaces();

    println!("Interfaces disponibles :");
    for (index, interface) in interfaces.iter().enumerate() {
        println!("{}: {}", index, interface.name);
    }

    let mut choice = String::new();
    println!("Veuillez choisir une interface à sniffer:");
    io::stdin()
        .read_line(&mut choice)
        .expect("Erreur lors de la lecture de l'input");

    let choice: usize = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Sélection invalide");
            return None;
        }
    };

    if choice >= interfaces.len() {
        println!("Sélection invalide");
        return None;
    }

    let interface = &interfaces[choice];
    println!("Vous avez choisi l'interface: {}", interface.name);

    Some(interface.name.clone())
}

#[tauri::command(rename_all = "snake_case")]
fn get_interfaces() -> Vec<String> {
    
    let interfaces = datalink::interfaces();
    info!("Fetching network interfaces");

    let names: Vec<String> = interfaces.iter().map(|iface| {
        let name = iface.name.clone();
        info!("Found interface: {}", name);
        name
    }).collect();

    names
}

