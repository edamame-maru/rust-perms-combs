use std::io::{self, Write};

fn main() {
    println!("Enter values for n, r");

    print!("n: ");
    io::stdout().flush().unwrap();

    let mut buffer_p = String::new();

    io::stdin()
        .read_line(&mut buffer_p)
        .expect("Failed to read line");

    let n: u32 = buffer_p
        .trim()
        .parse()
        .expect("Failed to parse string");


    print!("r: ");
    io::stdout().flush().unwrap();

    let mut buffer_c = String::new();

    io::stdin()
        .read_line(&mut buffer_c)
        .expect("Failed to read line");

    let r: u32 = buffer_c
        .trim()
        .parse()
        .expect("Failed to parse string");

    println!();
    println!("nPr: {}P{} = {}", n, r, perm(n, r));
    println!("nCr: {}C{} = {}", n, r, comb(n, r));

}


fn perm(n: u32, r: u32) -> u32 {
    fctrl(n) / fctrl(n - r)
}

fn comb(n: u32, r: u32) -> u32 {
    fctrl(n) / (fctrl(r) * fctrl(n - r))
}

fn fctrl(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * fctrl(n - 1)
    }
}


