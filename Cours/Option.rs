fn main() {
    // Une valeur optionnelle, comme si elle venait d'une configuration
    let maybe_number: Option<i32> = Some(10);

    // 1. map — double le nombre s'il est présent
    let doubled = maybe_number.map(|n| n * 2);

    // 2. and_then — on ne garde le résultat que s'il est inférieur à 50
    let checked = doubled.and_then(|n| {
        if n < 50 {
            Some(n)
        } else {
            None
        }
    });

    // 3. unwrap_or — si le résultat est None, on fournit une valeur par défaut
    let final_value = checked.unwrap_or(0);

    // 4. unwrap — ici, juste pour montrer : on sait que maybe_number est Some
    //    let original_value = maybe_number.unwrap(); // panique si None
    let original_value = maybe_number.unwrap();

    println!("Valeur d'origine : {}", original_value);
    println!("Valeur finale : {}", final_value);
}
