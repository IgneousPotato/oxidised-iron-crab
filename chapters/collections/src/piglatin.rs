pub fn to_pig_latin(s: &String) -> String {

    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let first_char = &s
                    .to_lowercase()
                    .chars()
                    .nth(0)
                    .unwrap();

    if vowels.contains(first_char) {
        format!("{s}-hay")
    } else {
        format!("{}-{}ay", &s[1..], first_char)
    }
}