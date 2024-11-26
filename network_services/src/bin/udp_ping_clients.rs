//client udp qui envoie PING a un server dont l'adresse IP et le port sont passes en ligne de commande

use std::env;
use std::net::UdpSocket;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <ip>:<port> <message>", args[0]);
        std::process::exit(1);
    }

    let ip_port = &args[1];
    let message = &args[2];

    let socket = UdpSocket::bind(ip_port).expect("Could not bind socket");

    let server_addr = ip_port.parse::<std::net::SocketAddr>().expect("Invalid IP address and port");
    socket.send_to(message.as_bytes(), server_addr).expect("Failed to send message");


    let mut buf = [0; 1024];
    let (amt, _src) = socket.recv_from(&mut buf).expect("Failed to receive message");
    println!("Received message: {}", String::from_utf8_lossy(&buf[..amt]));
    
}

