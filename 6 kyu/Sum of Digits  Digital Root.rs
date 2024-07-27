fn digital_root(n: i64) -> i64 {
    let mut num = n;
    let mut sum = 0;

    while num > 0 {
        sum += num % 10; 
        num /= 10; 
    }
    
    while sum >= 10 {
        num = sum;
        sum = 0;
        
        while num > 0 {
            sum += num % 10; 
            num /= 10; 
        }
    }

    sum
}