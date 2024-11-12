fn main() {
    let string1 = "Bonjour Limoges";
    let string2 = "";
    
    print_first_word1(string1);
}

//fonction qui prend en argument une ref vers une chaine et qui affiche son premier mot sous la forme suivante
fn print_first_word1(phrase: &str) {
    // Diviser la chaîne en mots et récupérer le premier
    match phrase.split_whitespace().next() {
        Some(premier_mot) => println!("Premier mot: {}", premier_mot),
        None => println!("La chaîne est vide ou ne contient pas de mots."),
    }
}





