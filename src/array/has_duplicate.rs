use std::collections::HashMap;

pub fn has_duplicate(nums: &[i32]) -> bool {
    let mut map = HashMap::new();
    for n in nums {
        if !map.contains_key(n) {
            map.entry(n).or_insert(true);
        } else {
            return true
        }
    }

    return false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums: [i32;10] = [1,2,3,4,4,6,7,8,9,10];
        let mut result = has_duplicate(&nums);
        assert_eq!(result, true);

        let nums = [1,2,3,4,5,6,7,8,9,10];
        result = has_duplicate(&nums);
        assert_eq!(result, false);
    }
}