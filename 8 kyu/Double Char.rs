fn double_char(s: &str) -> String {
    let mut string = String::new();

    for c in s.chars(){
         string.push(c);
         string.push(c);        
    }
    
    string
}