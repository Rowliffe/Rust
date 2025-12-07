fn prix_final(prix: f64, reduction: Option<u8>) -> f64 {
    reduction
        .map(|r| prix - (prix * (r as f64 / 100.0)))   // Applique la réduction si elle existe
        .unwrap_or(prix)                                // Sinon, retourne le prix d'origine
}

fn main() {
    let produit1 = prix_final(100.0, Some(20)); // Réduction de 20%
    let produit2 = prix_final(80.0, None);      // Aucune réduction

    println!("Prix final du produit 1 : {}", produit1);
    println!("Prix final du produit 2 : {}", produit2);
    
}

