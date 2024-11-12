fn main() {
    let string1 = "-17";
    let string2 = "Tux";
    
    convert_to_int1(string1);
    convert_to_int1(string2);
}

fn convert_to_int1(chaine: &str) -> i32 {
    // Convertir la chaîne en entier, et affiche son carré si c'est impossible affiche n'est pas entier
    match chaine.parse::<i32>() {
        Ok(nombre) => {
            println!("Le carré de {} est {}", nombre, nombre * nombre);
            nombre
        }
        Err(_) => {
            println!("{} n'est pas un entier", chaine);
            0
        }
    }
    
}
