fn main() {
    let string1 = "-17";
    let string2 = "Tux";

    convert_to_int1(string1);
    convert_to_int1(string2);
    
    convert_to_int3(string1);
    convert_to_int3(string2); 
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

//fonction convert_to_int2 qui prend en argument une reference vers une chaine et qui renvoie son entier et son carre ou panique s'il n'y en a pas avec .expect()
fn convert_to_int2(chaine: &str) -> i32 {
    // Convertir la chaîne en entier
    let nombre = chaine.parse::<i32>().expect("La chaîne n'est pas un entier");
    println!("Le carré de {} est {}", nombre, nombre * nombre);
    nombre
}

//fonction convert_to_int3 qui prend en argument une reference vers une chaine et qui renvoie son entier et renvoi un resultat de type anyhow::Result<> si ca reussi on affiche le carre sinon on propage l'erreur par ?
fn convert_to_int3(chaine: &str) -> anyhow::Result<i32> {
    // Convertir la chaîne en entier
    let nombre = chaine.parse::<i32>()?;
    println!("Le carré de {} est {}", nombre, nombre * nombre);
    Ok(nombre)
}