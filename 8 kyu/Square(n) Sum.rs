fn square_sum(vec: Vec<i32>) -> i32 {
  let sum_of_powers: i32 = vec.iter().map(|&x| x.pow(2 as u32)).sum();
    sum_of_powers
}