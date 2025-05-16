use http_core::add1;

fn main() {
    println!("Hello, world!");
    let var = add1(1,2);
    dbg!(&var);
}
