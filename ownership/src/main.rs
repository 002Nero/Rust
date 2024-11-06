fn main() {
    let float1 = 5.15;
    let float2 = 3.56;
    let result_average = average(float1, float2); // c'est ici que les valeurs sont passees a la fonction
    println!("La moyenne de {} et {} est {}", float1, float2, result_average);
    //---------------------------------------------------------------------------//

    let rect = Rectangle {
        length: 9.0,
        width: 7.0,
    };

    let result_perimeter = perimeter(rect.clone());
    println!("Le périmètre du rectangle est: {}", result_perimeter);
    let result_perimeter = perimeter(rect.clone());
    println!("Le périmètre du rectangle est: {}", result_perimeter);

    let mut x = 3.5;
    let mut y = 7.2;

    println!("Avant l'échange : x = {}, y = {}", x, y);

    // Appel de la fonction swap pour échanger les valeurs de x et y
    swap(&mut x, &mut y);

    println!("Après l'échange : x = {}, y = {}", x, y);
    println!("Avant l'échange : x = {}, y = {}", x, y);

    // Appel de la fonction swap pour échanger les valeurs de x et y
    swap(&mut x, &mut y);

    println!("Après l'échange : x = {}, y = {}", x, y);
}
//Definir une fonction average() qui prends 2 flottants f64 en parametre et revoie leur moyenne
fn average(premierfloat: f64, secondfloat: f64) -> f64 {
    (premierfloat + secondfloat) / 2.0
}

//------------------------------------------------------------------------------//

//Definissez la structure suivante
#[derive(Clone)]
#[derive(Copy)]
struct Rectangle {
    length : f64,
    width : f64,
}

fn perimeter(rect: Rectangle) -> f64 {
    2.0 * (rect.length + rect.width)
}

//----------------------------------------------------------------------------//

fn swap(a: &mut f64, b: &mut f64) {
    let temp = *a;
    *a = *b;
    *b = temp;
}