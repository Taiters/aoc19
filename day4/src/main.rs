use std::ops::Range;
use std::collections::HashMap;

fn find_matches(range: Range<u32>) -> Vec<u32> {
    let mut results = Vec::new();
    let range: Vec<u32> = range.collect();
    let mut index = 0;
    loop {
        if index >= range.len() {
            break;
        }

        let i = range[index];
        let mut s = i.to_string();
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
            index += 1;
        } else {
            match s_bytes.windows(2).enumerate().find(|(idx, values)| values[1] < values[0]) {
                Some((idx, _)) => {
                    let value = s.get(idx..idx+1).unwrap();
                    let value = value.repeat(s.len() - idx -1);
                    s.replace_range(idx+1..s.len(), &value);

                    let jump: usize = (&s.parse().unwrap() - i) as usize;
                    index += jump;
                }
                None => index += 1
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