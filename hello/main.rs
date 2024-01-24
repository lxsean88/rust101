mod mod1;
mod mod2;

fn main() {
    println!("Hello Sean!");
    mod1::fn2();
    // mod1::fn1(); // error: fn1 is private
    mod2::fnmod2();
    mod2::another_mod::another();
}