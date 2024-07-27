use std::collections::HashMap;
fn find_odd(arr: &[i32]) -> i32 {
    let mut frequency_map = HashMap::new();

    for &element in arr {
        let counter = frequency_map.entry(element).or_insert(0);
        *counter += 1;
    }

    for (&element, &count) in &frequency_map {
        if count % 2 != 0 {
            return element;
        }
    }

    0
    
}
