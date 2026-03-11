pub fn staircase(n: i32) -> String {
    let mut result = String::new();

    for i in 1..=n {
        let spaces = " ".repeat((n - i) as usize);
        let hashes = "#".repeat(i as usize);

        result.push_str(&(spaces + &hashes));

        if i != n {
            result.push('\n');
        }
    }

    result
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staircase() {
        let result = staircase(4);

        let expected =
"   #
  ##
 ###
####";

        assert_eq!(result, expected);
    }
}
