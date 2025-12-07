trait PetTrait {
    fn talk(&self) -> String;

    fn greet(&self) {
        println!("Oh you're a cutie! What's your name? {}", self.talk());
    }
}
struct Dog {
    name : String

}
struct Cat {
    name : String
}


impl PetTrait for Dog { 
    fn talk(&self) -> String {
        format!("Woof, my name is {}!", self.name)
    }

}
impl PetTrait for Cat {
    fn talk(&self) -> String {
        format!("MIAOUUU, my name is {}!", self.name)
    }
}

fn print_pet_name<T : PetTrait>(pet : &T ) -> T {
    println!("The name of the animal is : {}",pet.name());
}

fn main (){
    let cat = Cat { name : String::from("Caramel")};
    let cat = Dog { name : String::from("Boxer")};

    print_pet_name(&cat);
    print_pet_name(&dog);
}