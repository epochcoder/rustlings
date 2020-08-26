// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` for hints!

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let cats = vec![("Nala", 21), ("Ninja", 22)];
    let (name, age) = cat;

    show_cats(&cats);

    println!("vec length is: {}", (*cats).len());

    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    println!("{} is {} years old.", name, age);
}

fn show_cats(cats: &Vec<(&str, i32)>) -> () {
    for (naam, ouderdom) in cats {
        println!("{} is {} jaar oud", naam, ouderdom);
    }
    ;
}
