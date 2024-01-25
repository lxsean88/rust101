mod mod1;
mod mod2;
fn add_things() -> u32{
    let mut sumi:u32 = 0;
    for i in 1..101 { //upper bound is exclusive, lower bound is inclusive
        sumi += i;
    }
    sumi as u32
}

fn modifies(x: &mut i32) {
    *x += 1;
}

fn main() {
    println!("Hello Sean!");
    mod1::fn2();
    // mod1::fn1(); // error: fn1 is private
    mod2::fnmod2();
    mod2::another_mod::another();

    //basic loop, add 1 to 100
    println!("sum: {:?}", add_things());

    //modify fn parameter
    println!("modified value: {:?}", {
        let mut x = 1;
        modifies(&mut x);
        x
    });
}