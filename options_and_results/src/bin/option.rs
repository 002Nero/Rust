fn main() {
    let string1 = "Bonjour Limoges";
    let string2 = "";
    
    iterate_over_words(string1);
    iterate_over_words(string2);
    
    

}

//fonction qui prend en argument une ref vers une chaine et qui affiche son premier mot sous la forme suivante

// fn print_first_word1(string: &str) {
//     // Diviser la chaîne en mots et récupérer le premier
//     match string.split_whitespace().next() {
//         Some(premier_mot) => println!("Premier mot: {}", premier_mot),
//         None => println!("La chaîne est vide ou ne contient pas de mots."),
//     }
// }

//fonction print_first_word2 qui prend en argument une reference vers une chaine et qui renvoie son premier mot ou panique s'il n'y en a pas

// fn print_first_word2(string: &str) {
//     // Séparer la chaîne en mots et prendre le premier mot, s'il y en a un
//     // Diviser la chaîne en mots et récupérer le premier avec `.expect()`
//     let premier_mot = string
//         .split_whitespace()
//         .next()
//         .expect("La chaîne est vide ou ne contient pas de mots.");
// 
//     println!("Premier mot: {}", premier_mot);
// }



//fonction iterate_over_words() qui prend en argument une reference vers une chaine et qui affiche chacun de ses mots sur une ligne
fn iterate_over_words(phrase: &str) {
    // Diviser la chaîne en mots et itérer sur chacun d'eux
    for mot in phrase.split_whitespace() {
        println!("{}", mot);
    }
}





