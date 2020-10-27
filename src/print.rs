pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic formatting
    println!("{} is {}", "Brad", 21);

    // Can't do this - println!(21); - must do println!("{}", 21);

    // Positional Arguments
    println!("{0} is {1} and {0} likes to {2}", "Brad", 21, "code");

    // Named arguments
    println!("{name} likes to play {game}", name = "Brad", game = "video games");

    // Placeholder Traits
    println!("Binary: {:b} | Hex: {:x} | Octol: {:o}", 10, 10, 10);

    // Placeholder for debug trait (tuple)
    println!("{:?}", (12, true, "hi"));

    // Basic Maths
    println!("10 + 10 = {}", 10 + 10);
}