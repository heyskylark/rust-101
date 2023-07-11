fn main() {
    let s = String::from("Test string.");

    let s_part = first_part(&s);

    println!("Whole phrase: {s}");
    println!("First word: {s_part}");

    final_print(s);
}

fn first_part(s: &String) -> &str {
    let s_bytes = s.as_bytes();

    for (i, &part) in s_bytes.iter().enumerate() {
        if part == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn final_print(s: String) {
    let new_part = first_part(&s);
    println!("Final print part: {new_part}");
}
