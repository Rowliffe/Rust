fn analyse_phrase(phrase: &str) -> (usize, &str, f64) {
    if phrase.is_empty() {
        return (0, "", 0.0);
    }
    let nombre_de_mots: usize = phrase.split_whitespace().count();
    let mot_le_plus_long: &str = phrase.split_whitespace().max_by_key(|mot| mot.len()).unwrap_or("");
    let longueur_moyenne: f64 = phrase.split_whitespace().map(|mot| mot.len() as f64).sum::<f64>() / nombre_de_mots as f64;
    return (nombre_de_mots, mot_le_plus_long, longueur_moyenne);
}
fn main() {
    let phrase: &str = "Bonjour, comment ça va ?";
    let (nombre_de_mots, mot_le_plus_long, longueur_moyenne) = analyse_phrase(phrase);
    println!("Nombre de mots: {}", nombre_de_mots);
    println!("Mot le plus long: {}", mot_le_plus_long);
    println!("Longueur moyenne: {}", longueur_moyenne);
}