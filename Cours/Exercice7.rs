fn traiter_notes(mut notes: Vec<u8>) {
    // 1. Affiche la moyenne
    let somme: u32 = notes.iter().map(|&n| n as u32).sum();
    let moyenne = somme as f32 / notes.len() as f32;
    println!("Moyenne : {:.2}", moyenne);

    // 2. Trie les notes dans l’ordre décroissant
    notes.sort_by(|a, b| b.cmp(a));

    // 3. Retire les notes < 10
    notes.retain(|&n| n >= 10);

    // 4. Ajoute une nouvelle note (15)
    notes.push(15);

    // 5. Affiche les notes restantes une par une
    println!("Notes restantes :");
    for n in notes {
        println!("{}", n);
    }
}

fn main() {
    let notes = vec![12, 8, 19, 7, 14, 10];
    traiter_notes(notes);
}