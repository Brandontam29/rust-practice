fn main() {
    println!("loop loop start");
    let mut i = 0;
    loop {
        i += 1;

        println!("for loop i = {}", i);

        if i >= 5 {
            break;
        };
    }

    println!("while loop start");
    let mut j = 0;
    while j < 5 {
        j += 1;
        println!("while loop j = {}", j);
    }

    println!("for loop start");
    for n in 1..=5 {
        println!("for loop n = {}", n);
    }
}
