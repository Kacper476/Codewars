use std::io;

fn main() {
    // Declare a mutable string variable
    let mut s = String::from("sadasd");
    
    // Concatenate a string slice to the mutable string
    s.push_str("ss");
    
    // Print a message
    println!("You");
}


fn double_char(s: &str) -> String {
    let mut string = String::new();
    let mut x=1;
    for c in s.chars(){
         string.
        
    }
    
    string.push(x.to_string());
}

fn dna_to_rna(dna: &str) -> String {
    dna.to_string()
}