use std::fs::File;
use std::iter::FromIterator;
use std::process::Command;
use std::{io, io::prelude::*};

fn main() {
    // Hello world ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
    // Print until loop to 0 +[.+]

    let mut program_source: String = String::new();
    io::stdin()
        .read_line(&mut program_source)
        .expect("Failed to get instructions");

    let mut instructions = String::from_iter(program_source.chars().map(|char| match char {
        '>' => "++ptr;",            // Increases program pointer.
        '<' => "--ptr;",            // Decreases program pointer.
        '+' => "++*ptr;",           // Increases element under program pointer.
        '-' => "--*ptr;",           // Decreases element under program pointer.
        '.' => "putchar(*ptr);",    // Outputs ASCII code under program pointer.
        ',' => "*ptr = getchar();", // Reads char from io::stdin and stores char in ram_stack[ram_pointer].
        '[' => "while(*ptr){",      // Starts loop, flag under program pointer.
        ']' => "}",                 // Indicates end of loop.
        _ => "",                    // Symbol removed.
    }));
    instructions = format!("#include <stdio.h>\nint main(){{char array[30000] = {{0}};char *ptr = array;{instructions}return 0;}}");

    File::create("BrainFuck.c")
        .expect("Failed to create source file BrainFuck.c")
        .write_all(instructions.as_bytes())
        .expect("Failed to write to source file");

    Command::new("gcc")
        .arg("-o")
        .arg("BrainFuck")
        .arg("-march=native")
        .arg("-O3")
        .arg("BrainFuck.c")
        .output()
        .expect("Failed to compile source (depends on gcc)");

    let execution = Command::new(".\\BrainFuck.exe")
        .output()
        .expect("Failed to execute generated executable");

    println!("{}", String::from_utf8_lossy(&execution.stdout));
}
