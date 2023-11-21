pub fn run() {
    // print to console
    println!("Hello from print.rs");

    // print number, need to use placeholder
    println!("Number: {}", 1);
    
    // Basic formatting
    println!("{} pa deni {}, {} vari", "Dael", "hai", 150);
    
    // Position Arguments
    println!("{0} hi {0} pa deni {1}, {2} vari", "Dael", "hai", 150);
    
    // Position Arguments
    println!("{name} likes to {activity}",
        name="Dael Singh",
        activity="Code"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello")); // this is tupple

    println!("10 + 10 = {} - 1", 10 + 11);


}