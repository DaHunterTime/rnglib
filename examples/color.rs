use rnglib::random;

fn main() {
    let mut rng = random!();

    let r = rng.randrange(0..256);
    let g = rng.randrange(0..256);
    let b = rng.randrange(0..256);

    // Prints 4 spaces with a random background color using ANSI escape sequences.
    println!("\x1b[48;2;{r};{g};{b}m    \x1b[0m #{r:0>2x}{g:0>2x}{b:0>2x}");
}
