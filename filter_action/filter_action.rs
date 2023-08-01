
fn main() {
    let mut input: Vec<String> = vec!["Zebra", "nice camel", "horse"].iter().map(|&s| String::from(s)).collect();
    let mut filter = |s: &String| { true };
    // let mut filter = |s: &String| { s.chars().next().unwrap().is_uppercase() };
    if true { filter = |s: &String| { s.chars().next().unwrap().is_uppercase() }; }

    // let result: Vec<&String> = input.iter().filter(|&s| s.chars().next().unwrap().is_uppercase()).collect();
}
