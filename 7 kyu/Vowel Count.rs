fn get_count(string: &str) -> usize {
    string.chars()
          .filter(|&c| c == 'a'|| c == 'e'|| c == 'i'|| c == 'o'|| c == 'u')
          .count()
}