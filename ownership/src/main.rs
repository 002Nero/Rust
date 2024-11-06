fn main() {

    
    let float1 = 5.15;
    let float2 = 3.56;
    let result = average(float1, float2); // c'est ici que les valeurs sont passees a la fonction
    println!("La moyenne de {} et {} est {}", float1, float2, result);
    let result = average(float1, float2);
    println!("La moyenne de {} et {} est {}", float1, float2, result);




}

//Definir une fonction average() qui prends 2 flottants f64 en parametre et revoie leur moyenne
fn average(premierfloat: f64, secondfloat: f64) -> f64 {
    (premierfloat + secondfloat) / 2.0
}
