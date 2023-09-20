use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::Packet;
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::arp::ArpPacket;
use pnet::packet::dhcp::DhcpPacket;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
//use pnet::packet::ip::IpNextHeaderProtocol;


use std::io;

use serde::Serialize;

#[derive(Debug, Serialize)]
struct Relations {
    destination: String,
    source: String,
    count: u64,
}

extern crate csv;

use std::collections::HashSet;
use std::fs::File;
#[derive(Debug, Serialize, Hash, Eq, PartialEq, Clone)]
struct PacketInfo {
    ethertype: String,
    source: String,
    destination: String,
    // ... add more fields as needed
}

fn main() {
    let mut observed_packets = HashSet::new();  // To keep track of unique packets
    let mut wtr = csv::Writer::from_writer(File::create("packets.csv").unwrap());

    // Écrivez les en-têtes
    wtr.write_record(&["EtherType", "IPv6 Source", "IPv6 Destination"]).unwrap();


    if let Some(interface_name) = choose_interface() {
        println!("L'interface choisie est: {}", &interface_name);
        let interface_names_match = |iface: &NetworkInterface| iface.name == interface_name;

    let interfaces = datalink::interfaces();
    let interface = match interfaces.into_iter().filter(interface_names_match).next() {
        Some(interface) => interface,
        None => {
            eprintln!("No such interface '{}'", interface_name);
            return;
        }
    };

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("An error occurred when creating the datalink channel: {}", e),
    };
    println!("Start reading packet: ");

    loop {
        match rx.next() {
            Ok(packet) => {
                if let Some(ethernet_packet) = EthernetPacket::new(packet) {
                    match ethernet_packet.get_ethertype() {

                        pnet::packet::ethernet::EtherTypes::Ipv6 => {
                            if let Some(ipv6_packet) = Ipv6Packet::new(ethernet_packet.payload()) {
                                let info = PacketInfo {
                                    ethertype: format!("  EtherType: {}", ethernet_packet.get_ethertype()),
                                    source: format!("  IPv6 Source: {}", ipv6_packet.get_source()),
                                    destination: format!("  IPv6 Destination: {}", ipv6_packet.get_destination())
                                };
                                if !observed_packets.contains(&info) {
                                    println!("New unique packet: {:?}", &info);
                                    observed_packets.insert(info.clone());
                                    wtr.serialize(info).unwrap();
                                    wtr.flush().unwrap(); // Assurez-vous que les données sont écrites
                                }
                                println!("  EtherType: {}", ethernet_packet.get_ethertype());
                                println!("  IPv6 Source: {}", ipv6_packet.get_source());
                                println!("  IPv6 Destination: {}", ipv6_packet.get_destination());
                                println!("  traffic_class: {}", ipv6_packet.get_traffic_class());
                                println!("  flow_label: {}", ipv6_packet.get_flow_label());
                                println!("  next_header: {}", ipv6_packet.get_next_header());
                                if ipv6_packet.get_next_header() == pnet::packet::ip::IpNextHeaderProtocols::Tcp {
                                    if let Some(tcp_packet) = TcpPacket::new(ethernet_packet.payload()) {
                                        let source_port = tcp_packet.get_source();
                                        let destination_port = tcp_packet.get_destination();
                    
                                        println!("    Source port: {}, Destination port: {}", source_port, destination_port);
                                    }
                                }
                                if ipv6_packet.get_next_header() == pnet::packet::ip::IpNextHeaderProtocols::Udp {
                                    if let Some(udp_packet) = UdpPacket::new(ethernet_packet.payload()) {
                                        let source_port = udp_packet.get_source();
                                        let destination_port = udp_packet.get_destination();
                                        let checksum = udp_packet.get_checksum();
                    
                                        println!("    Source port: {}, Destination port: {}", source_port, destination_port);
                                        println!("    checksum: {}", checksum);
                                    }
                                }
                                if ipv6_packet.get_next_header() == pnet::packet::ip::IpNextHeaderProtocols::Icmpv6 {
                                    if let Some(icmpv6_packet) = UdpPacket::new(ethernet_packet.payload()) {
                                        let source_port = icmpv6_packet.get_source();
                                        let destination_port = icmpv6_packet.get_destination();
                    
                                        println!("    Source port: {}, Destination port: {}", source_port, destination_port);
                                    }
                                }
                                if ipv6_packet.get_next_header() == pnet::packet::ip::IpNextHeaderProtocols::Hopopt {
                                    if let Some(icmpv6_packet) = UdpPacket::new(ethernet_packet.payload()) {
                                        let source_port = icmpv6_packet.get_source();
                                        let destination_port = icmpv6_packet.get_destination();
                    
                                        println!("    Source port: {}, Destination port: {}", source_port, destination_port);
                                    }
                                }
                                println!("  version: {}", ipv6_packet.get_version());
                                // Add more IPv6 specific code here...
                                println!("---");
                            }
                        }

                        pnet::packet::ethernet::EtherTypes::Ipv4 => {
                            if let Some(ipv4_packet) = Ipv4Packet::new(ethernet_packet.payload()) {
                                let info = PacketInfo {
                                    ethertype: format!("  EtherType: {}", ethernet_packet.get_ethertype()),
                                    source: format!("  IPv4 Source: {}", ipv4_packet.get_source()),
                                    destination: format!("  IPv4 Destination: {}", ipv4_packet.get_destination())
                                };
                                if !observed_packets.contains(&info) {
                                    println!("New unique packet: {:?}###########################", &info);
                                    observed_packets.insert(info.clone());
                                    wtr.serialize(info).unwrap();
                                    wtr.flush().unwrap(); // Assurez-vous que les données sont écrites
                                }
                                //println!("MAC Source: {}", ethernet_packet.get_source());
                                //println!("MAC Destination: {}", ethernet_packet.get_destination());
                                //println!("Packet: {:?}", ethernet_packet.packet());
                                //Sprintln!("Payload: {:?}", ethernet_packet.payload());
                                println!("  EtherType: {}", ethernet_packet.get_ethertype());
                                println!("  IPv4 Source: {}", ipv4_packet.get_source());
                                println!("  IPv4 Destination: {}", ipv4_packet.get_destination());
                                println!("  Total length: {}", ipv4_packet.get_total_length());
                                println!("  flag: {}", ipv4_packet.get_flags());
                                println!("  next_level_protocol: {}", ipv4_packet.get_next_level_protocol());
                                if ipv4_packet.get_next_level_protocol() == pnet::packet::ip::IpNextHeaderProtocols::Tcp {
                                    if let Some(tcp_packet) = TcpPacket::new(ethernet_packet.payload()) {
                                        let source_port = tcp_packet.get_source();
                                        let destination_port = tcp_packet.get_destination();
                    
                                        println!("    Source port: {}, Destination port: {}", source_port, destination_port);
                                    }
                                }
                                if ipv4_packet.get_next_level_protocol() == pnet::packet::ip::IpNextHeaderProtocols::Udp {
                                    if let Some(udp_packet) = UdpPacket::new(ethernet_packet.payload()) {
                                        let source_port = udp_packet.get_source();
                                        let destination_port = udp_packet.get_destination();
                    
                                        println!("    Source port: {}, Destination port: {}", source_port, destination_port);
                                    }
                                }
                                if ipv4_packet.get_next_level_protocol() == pnet::packet::ip::IpNextHeaderProtocols::Icmpv6 {
                                    if let Some(icmpv6_packet) = UdpPacket::new(ethernet_packet.payload()) {
                                        let source_port = icmpv6_packet.get_source();
                                        let destination_port = icmpv6_packet.get_destination();
                    
                                        println!("    Source port: {}, Destination port: {}", source_port, destination_port);
                                    }
                                }
                                println!("  version: {}", ipv4_packet.get_version());
                                println!("  identification: {}", ipv4_packet.get_identification());

                                // Add more IPv4 specific code here...
                                println!("---");
                            }
                        }

                        pnet::packet::ethernet::EtherTypes::Arp => {
                            if let Some(arp_packet) = ArpPacket::new(ethernet_packet.payload()) {
                                //println!("MAC Source: {}", ethernet_packet.get_source());
                                //println!("MAC Destination: {}", ethernet_packet.get_destination());
                                //println!("Packet: {:?}", ethernet_packet.packet());
                                //Sprintln!("Payload: {:?}", ethernet_packet.payload());
                                println!("  EtherType: {}", ethernet_packet.get_ethertype());
                                println!("  hardware_type: {:?}", arp_packet.get_hardware_type());
                                println!("  operation: {:?}", arp_packet.get_operation());
                                println!("  protocol_type: {}", arp_packet.get_protocol_type());
                                println!("  target_hw_addr: {}", arp_packet.get_target_hw_addr());
                                println!("  Sender_hw_addr: {}", arp_packet.get_sender_hw_addr());

                                // Add more IPv4 specific code here...
                                println!("---");
                            }
                        }
                        pnet::packet::ethernet::EtherTypes::Ipx => {
                            if let Some(dhcp_packet) = DhcpPacket::new(ethernet_packet.payload()) {
                                //println!("MAC Source: {}", ethernet_packet.get_source());
                                //println!("MAC Destination: {}", ethernet_packet.get_destination());
                                //println!("Packet: {:?}", ethernet_packet.packet());
                                //Sprintln!("Payload: {:?}", ethernet_packet.payload());
                                println!("  EtherType: {}", ethernet_packet.get_ethertype());
                                println!("  htype: {:?}", dhcp_packet.get_htype());
                                println!("  flags: {:?}", dhcp_packet.get_flags());
                                println!("  giaddr: {}", dhcp_packet.get_giaddr());
                                println!("  ciaddr: {}", dhcp_packet.get_ciaddr());
                                println!("  yiaddr: {}", dhcp_packet.get_yiaddr());

                                // Add more IPv4 specific code here...
                                println!("---");
                            }
                        }
                        _ => {
                            println!("Unknown EtherType");
                            println!("---");
                        }
                    }
                }
            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
    } else {
        println!("Aucune interface valide n'a été choisie. Le programme va se terminer.");
    }
}

fn choose_interface() -> Option<String> {
    let interfaces = datalink::interfaces();
    
    println!("Interfaces disponibles :");
    for (index, interface) in interfaces.iter().enumerate() {
        println!("{}: {}", index, interface.name);
    }

    let mut choice = String::new();
    println!("Veuillez choisir une interface à sniffer:");
    io::stdin().read_line(&mut choice).expect("Erreur lors de la lecture de l'input");
    
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