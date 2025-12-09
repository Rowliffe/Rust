// Personne composant le groupe Mathéo BELLONNET, Naël MEIGNANT, ENZO CLAMY--COURTIAL
use std::io;

// correspond à l'interface de la bibliothèque
trait Library {
    fn add_book(
        &mut self,
        title: String,
        author: String,
        publication_date: u32,
        genre: Vec<String>
    );
    fn view_all_books_available(&self);
    fn view_all_books(&self);
}

// correspond à l'interface d'un livre
trait OneBook {
    fn return_book(&mut self);
    fn remove_book(&mut self);
}

// comme un getter / setter
struct Bookshelf {
    books: Vec<Book>,
}

struct Book {
    title: String,
    author: String,
    publication_date: u32,
    genre: Vec<String>,
    availability: bool,
}

impl Library for Bookshelf {
    //ajoute un livre à la bibliothèque
    fn add_book(
        &mut self,
        title: String,
        author: String,
        publication_date: u32,
        genre: Vec<String>
    ) {
        // vérifie si le livre existe déjà par son titre
        if self.books.iter().any(|b| b.title == title) {
            println!("Erreur : Le livre '{}' existe déjà ", title);
        } else {
            let book = Book {
                title,
                author,
                publication_date,
                genre,
                availability: true,
            };
            self.books.push(book);
            println!("Livre ajouté avec succès !");
        }
    }
    // affiche tous les livres disponibles
    fn view_all_books_available(&self) {
        for book in &self.books {
            if book.availability {
                println!(
                    "titre : {} | auteur.ice : {} | date de publication : {} | genre : {:?}",
                    book.title,
                    book.author,
                    book.publication_date,
                    book.genre
                );
            }
        }
    }
    // affiche tous les livres
    fn view_all_books(&self) {
        for book in &self.books {
            println!(
                "titre : {} | auteur.ice : {} | date de publication : {} | genre : {:?} | disponibilité : {}",
                book.title,
                book.author,
                book.publication_date,
                book.genre,
                if book.availability {
                    "Disponible"
                } else {
                    "Indisponible"
                }
            );
        }
    }
}

impl OneBook for Book {
    // retire un livre de la bibliothèque
    fn remove_book(&mut self) {
        if self.availability {
            self.availability = false;
            println!("Vous avez pris le livre '{}'\n", self.title);
        } else {
            println!("Le livre n'est déjà pas disponible")
        }
    }
    // retourne un livre à la bibliothèque
    fn return_book(&mut self) {
        if self.availability == true {
            println!("Le livre est déjà disponible");
        } else {
            self.availability = true;
            println!("Vous avez retourné le livre '{}'\n", self.title);
        }
    }
}

fn main() {
    let mut shelf = Bookshelf { books: Vec::new() };

    loop {
        println!("=== Menu ===");
        println!("1. Ajouter un livre");
        println!("2. Prendre un livre dans la bibliothèque");
        println!("3. Retourner un livre à la bibliothèque");
        println!("4. Afficher tous les livres");
        println!("5. Afficher les livres seulement disponible");
        println!("6. Quitter");
        println!("\nVotre choix :");


        let mut input_choice = String::new();
        io::stdin().read_line(&mut input_choice).unwrap(); //on lit l'entrée utilisateur
        let input_choice_error = input_choice.trim().to_string(); //on retire les espaces inutiles
        let result = input_choice_error.parse::<u32>();  //on convertit le resultat en u32

        // on récupère les erreurs 
        if result.is_err() {
            println!("Choix non valide (1 à 6 seulement)\n");
            continue;
        }

        let choice: u32 = result.unwrap(); // on récupère le choix

        match choice {
            1 => {
                println!("=> Titre : ");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                let title = title.trim().to_string();

                println!("=> Auteur : ");
                let mut author = String::new();
                io::stdin().read_line(&mut author).unwrap();
                let author = author.trim().to_string(); 

                println!("=> Date de publication : ");
                let mut publication_date = String::new();
                io::stdin().read_line(&mut publication_date).unwrap();
                let input_publication_date_error = publication_date.trim().to_string();
                let result = input_publication_date_error.parse::<u32>(); //on convertit le resultat en u32

                if result.is_err() {
                    println!("Il faut un nombre");
                    continue;
                }

                let publication_date: u32 = result.unwrap();

                println!("=> Genre : ");
                let mut genre = String::new();
                io::stdin().read_line(&mut genre).unwrap();
                let genre = genre.trim().to_string();

                shelf.add_book(title, author, publication_date, vec![genre]);
            }
            2 => {
                println!("=> Entrez le titre du livre à prendre :");
                let mut remove = String::new();
                io::stdin().read_line(&mut remove).unwrap();
                let remove = remove.trim();

                let mut found = false;

                // cherche le livre par son titre
                for book in &mut shelf.books {
                    if book.title == remove {
                        book.remove_book();
                        found = true;
                        break;
                    }
                }

                if found == false {
                    println!("Aucun livre trouvé avec ce titre");
                }
            }

            3 => {
                println!("=> Entrez le titre du livre à retourner :");
                let mut book_return = String::new();
                io::stdin().read_line(&mut book_return).unwrap();
                let book_return = book_return.trim();

                let mut found = false;
                
                // cherche le livre par son titre
                for book in &mut shelf.books {
                    if book.title == book_return {
                        book.return_book();
                        found = true;
                        break;
                    }
                }

                if found == false {
                    println!("Pas de livre avec ce titre ");
                }
            }

            4 => {
                println!("Liste de tous les livres dans la bibliothèque:");
                shelf.view_all_books();
            }

            5 => {
                println!("Liste des livres qui sont disponibles dans la bibliothèque:");
                shelf.view_all_books_available();
            }

            6 => {
                println!("=>  Au revoir !");
                break;
            }

            _ => println!("Choix non valide (1 à 6 seulement)\n"),
        }
    }
}
