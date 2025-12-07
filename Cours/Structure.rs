struct Personne {
    nom: String,
    age: u8,
    email: String,
}
fn description(personne: &Personne) {
    println!("Personne: {}", personne.nom);
    println!("Age: {}", personne.age);
    println!("Email: {}", personne.email);
}

fn main() {
    let mut john = Personne {
        nom: String::from("John"),
        age: 30,
        email: String::from("john@example.com"),
    };
    description(&john);


    john.age = 31;
    description(&john);

    let nom =  String::from("Elon Musk");
    let age = 50;
    let email = String::from("elon@musk.com");
    let elon = Personne {
        nom,
        age,
        email,
    };
    description(&elon);
}