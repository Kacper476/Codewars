fn create_phone_number(numbers: &[u8]) -> String {

    
    let number = format!("({}{}{}) {}{}{}-{}{}{}{}",
        numbers[0],numbers[1],numbers[2],
        numbers[3],numbers[4],numbers[5],
        numbers[6],numbers[7],numbers[8]
        ,numbers[9]) ;
                
    number
    
}