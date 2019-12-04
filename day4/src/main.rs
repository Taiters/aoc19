use std::ops::Range;
use std::collections::HashMap;

fn find_matches(range: Range<u32>) -> Vec<u32> {
    let mut results = Vec::new();
    for i in range {
        let s = i.to_string();
        let s_bytes = s.as_bytes();
        let mut sorted_bytes: Vec<u8> = s_bytes.to_vec();
        sorted_bytes.sort();

        if s_bytes == &sorted_bytes[..] {
            let counts = sorted_bytes.iter().fold(HashMap::new(), |mut acc, i| {
                let count = acc.entry(i).or_insert(0);
                *count += 1;
                acc
            });

            match counts.values().find(|&&x| x == 2) {
                Some(_) => results.push(i),
                None => ()
            }
        }
    }

    results
}

fn main() {
    let matches = find_matches(172851..675870);
    println!("{}", matches.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_matches() {
        let result = find_matches(172851..222235);

        assert_eq!(result, vec!(
            177788,
            177799,
            177888,
            177889,
            177899,
            177999,
            178899,
            188899,
            188999,
            222233
        ));
    }
}