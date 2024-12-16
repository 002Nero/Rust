use std::env;
use std::net::UdpSocket;
use exquisite_corpse::add_line_and_make_response;

fn main() {
    // Récupère les arguments de la ligne de commande
    let args: Vec<String> = env::args().collect();
    // Vérifie que l'adresse IP et le port ont été fournis
    if args.len() != 2 {
        eprintln!("Usage: {} <ip>:<port>", args[0]);
        std::process::exit(1);
    }
// Crée un socket UDP et le lie à l'adresse spécifiée
    let ip_port = &args[1];
    let socket = UdpSocket::bind(ip_port).expect("Could not bind socket");

    let mut buf = [0; 1024];
    let mut text = String::new(); // Maintient le texte en mémoire

    loop {
        let (amt, src) = socket.recv_from(&mut buf).expect("Failed to receive message"); // Récupère le message
        let message = String::from_utf8_lossy(&buf[..amt]);// Convertit le message en String
        println!("Received message: {}", message);// Affiche le message reçu

        let response = add_line_and_make_response(&message, &mut text);// Ajoute la ligne au texte et renvoie le texte
        socket.send_to(response.as_bytes(), src).expect("Failed to send message");// Envoie le texte au client
    }
}