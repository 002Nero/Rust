//client ncat udp qui envoie au server ce qu'il recoit sur l'entree standard et qui affiche la sortie standard ce qui provient du server

use std::env;
use std::net::UdpSocket;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <ip>:<port>", args[0]);
        std::process::exit(1);
    }

    let ip_port = &args[1];

    let socket = UdpSocket::bind(ip_port).expect("Could not bind socket");

    let mut buf = [0; 1024];
    loop {
        let (amt, _src) = socket.recv_from(&mut buf).expect("Failed to receive message");
        let message = String::from_utf8_lossy(&buf[..amt]);
        println!("Received message: {}", message);

        let response = format!("PONG {}", message);
        socket.send_to(response.as_bytes(), _src).expect("Failed to send message");
    }
}
