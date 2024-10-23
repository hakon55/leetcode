use std::collections::HashMap;

fn roman_to_int(s: String) -> i32 {
    let mut map = HashMap::new();
    map.insert('I', 1);
    map.insert('V', 5);
    map.insert('X', 10);
    map.insert('L', 50);
    map.insert('C', 100);
    map.insert('D', 500);
    map.insert('M', 1000);

    let mut result = 0;
    let chars: Vec<char> = s.chars().collect();

    for i in 0..chars.len() {
        let current_value = map[&chars[i]];
        if i < chars.len() - 1 && current_value < map[&chars[i + 1]] {
            result -= current_value;
        } else {
            result += current_value;
        }
    }

    result
}

fn main() {
    let roman = String::from("XXVII");
    let integer = roman_to_int(roman);
    println!("The integer value is: {}", integer);
}
