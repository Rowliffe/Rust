enum Notification {
    Email { name: String, from: String, subject: String },
    SMS(String),
}

struct User {
    name: String,
    notification: Option<Notification>,
}

fn main() {
    let  user = User {
        name: String::from("John"),
        notification: Some(Notification::Email {
            name: String::from("Alice"),
            from: String::from("alice@example.com"),
            subject: String::from("Bienvenue !"),
        }),
    };

  
    if let Some(Notification::Email { name, from, subject }) = &user.notification {
        println!(
            "Email reçu de {name} <{from}>\nContenu : {subject}"
        );
    }

    match &user.notification {
        Some(Notification::Email { name, from, subject }) => {
            println!("Email de {name} <{from}> | Sujet : '{subject}'");
        }
        Some(Notification::SMS(msg)) => println!("SMS: {msg}"),
        None => println!("Aucune notification"),
    }

  
    let mut notifications = vec![
        Notification::Email {
            name: String::from("Bob"),
            from: String::from("bob@example.com"),
            subject: String::from("Salut !"),
        },
        Notification::SMS(String::from("RDV à 15h")),
        Notification::Email {
            name: String::from("Admin"),
            from: String::from("admin@example.com"),
            subject: String::from("Mise à jour"),
        },
    ];

    println!("\nSimulation de la file de notifications :");

    while let Some(notification) = notifications.pop() {
        match notification {
            Notification::Email { name, from, subject } => {
                println!("Email de {name} <{from}> | Sujet : {subject}");
            }
            Notification::SMS(msg) => {
                println!("SMS: {msg}");
            }
        }
    }
}
