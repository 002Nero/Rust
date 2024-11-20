//ecrire une fn async main qui attend les 4 parametres en argument ligne de commande et qui teste toute la plage de port entre port_min et port_max sur host et affiche les ports en utilisant is_open()

use clap::Parser;
use port_scanner::{is_open, Parameters};

#[tokio::main]
async fn main() {
    let params = Parameters::parse();

    for port in params.port_min..=params.port_max {
        if is_open(&params.host, port, params.timeout).await {
            println!("Port {} is open", port);
        }
    }
}