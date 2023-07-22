
fn is_anagram(string1: &str, string2: &str) -> bool {
    if string1.len() != string2.len() {
        return false
    }

    let mut compare = string2.to_uppercase();

    for c1 in  string1.to_uppercase().chars(){
        compare = compare.replacen(c1, "", 1);
    }

    if compare.len() > 0 {
        return false;
    }

    return true;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_works(){
        let result = is_anagram("YesNo", "NoYes");
        assert_eq!(result, true);

        let result = is_anagram("Hello", "OHELL");
        assert_eq!(result, true);

        let result = is_anagram("ABCD", "CDEF");
        assert_eq!(result, false);

        let result = is_anagram("123456789", "1234");
        assert_eq!(result, false)
    }
}