use std::collections::HashMap;

pub fn median(v: &Vec<i32>) -> i32 {
    let length = v.len();
    
    if length % 2 == 0 {     
        let middle = length / 2;
        return (v[middle - 1] + v[middle]) / 2;
    } else {
        let middle = (length - 1) / 2;
        return v[middle];
    }
}

pub fn mode(v: &Vec<i32>) -> i32 {
    let mut occurrences = HashMap::new();

    for &value in v {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .max_by(|key, count| key.1.cmp(&count.1))
        .map(|(k, _)| k)
        .unwrap()
}