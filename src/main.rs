use std::{cell::RefCell, rc::Rc};

struct Animal {
    name: String,
    height: f64,
}

fn new_animal_rc(name: &str, height: f64) -> Rc<RefCell<Animal>> {
    Rc::new(RefCell::new(Animal {
        name: name.to_string(),
        height,
    }))
}
fn main() {
    let whale = new_animal_rc("whale", 4.);
    let fox = new_animal_rc("fox", 0.6);
    let whale_clone = whale.clone();
    let animals= vec![whale, fox];
    let mut tall_animals;
    {
        tall_animals = animals.iter().filter(|&a| a.borrow().height > 1.);
    }
    let mut tall_animals_clone = tall_animals.clone();
    display_animals(&mut tall_animals);

    // NOK : the collection tall_animals_clone is not mutable
    // and it could not be mutable, being iterated at the same time
    // animals.push(new_animal_rc("beaver", 0.8));

    // Change the whale height, so that it is not present in the next query/display
    whale_clone.borrow_mut().height = 0.2;
    display_animals(&mut tall_animals_clone);
}

fn display_animals<'a, I: Iterator<Item = &'a Rc<RefCell<Animal>>>>(animals: &'a mut I) {
    for a in animals {
        println!("{}", a.borrow().name);
    }
}
