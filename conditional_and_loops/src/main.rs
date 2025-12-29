fn main() {
    let x = true ;
    if x {
        print!("x is true");
    }else if !x {
        print!("x is false");
    }else{
        print!("every thing is wrong")
    }


    for i in 0..10{  // the loop will run from 0 to 9 (10 - 1 )
        print!("{}", i);
    }

    let _z = 5 ; // _ means we know we wont be using it just ignore the warning
}
