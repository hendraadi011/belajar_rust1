use gete_say_hello::{say_goodbye, say_hello};

fn main() {
    let say = say_hello("Hallo");
    let good = say_goodbye("Hallo");

    println!("{}", say);
    println!("{}", good);
}
