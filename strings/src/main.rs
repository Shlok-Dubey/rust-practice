// string , Array, vectors are bit harder in Rust because they can changed at run time.
// they get store in heap instead of stack 
// their are many ways to initialise string in rust


fn main() {
    let name : String = String::from("Shlok");
    print!("{}",name);

    let char1 = name.chars().nth(1000);
    match char1 {
        Some(c)=> print!("{}",c),
        None => print!("Nothing exist at this position");
}
