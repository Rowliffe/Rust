fn analyse_phrase(phrase: &String) -> (usize, &str, f64) {
    let words: Vec<&str> = phrase
        .split_whitespace()   // crée un itérateur sur les mots
        .collect();           // collecte en Vec<&str> (slices)

    let mut longest_word: &str = "";
    let mut total_length: usize = 0;

    // Parcours pour trouver le plus long mot
    for word in &words {
        // &words est une référence partagée
        if word.len() > longest_word.len() {
            longest_word = word;   // shared reference
        }
    }

    // Parcours pour calculer la longueur totale
    for word in &words {
        total_length += word.len();
    }

    let average_length = if words.len() > 0 {
        total_length as f64 / words.len() as f64
    } else {
        0.0
    };

    (words.len(), longest_word, average_length)
}

fn main() {
    let phrase = String::from("Rust est un langage rapide et sûr");
    let (count, longest, average) = analyse_phrase(&phrase);
    println!("Nombre de mots : {}", count);
    println!("Mot le plus long : {}", longest);
    println!("Longueur moyenne : {:.2}", average);
}
