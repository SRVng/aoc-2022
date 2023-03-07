pub fn get_answer() {
    let input = include_str!("input.txt");

    let rucksack: Vec<String> = input.split("\n").map(|s| s.to_string()).collect();

    let compartments: Vec<Vec<String>> = rucksack
        .clone()
        .into_iter()
        .map(|r| {
            let mut first = r.clone();
            let rest = first.split_off(r.capacity() / 2);
            vec![first, rest]
        })
        .collect();
    dbg!(compartments
        .into_iter()
        .map(|v| intersect(v[0].clone(), v[1].clone(), None))
        .map(|c| get_char_map(c))
        .sum::<usize>());

    let group: Vec<Vec<String>> = rucksack
        .chunks(3)
        .map(|v| v.to_vec())
        .collect::<Vec<Vec<String>>>();

    dbg!(group
        .into_iter()
        .map(|v| intersect(v[0].clone(), v[1].clone(), Some(v[2].clone())))
        .map(|c| get_char_map(c))
        .sum::<usize>());
}

fn intersect(a: String, b: String, c: Option<String>) -> char {
    a.chars().fold('a', |mut acc, v| {
        b.chars().for_each(|ch| match c.clone() {
            Some(c) => c.chars().for_each(|chh| {
                if v.eq(&ch) && v.eq(&chh) {
                    acc = ch
                }
            }),
            None => {
                if v.eq(&ch) {
                    acc = ch
                }
            }
        });
        acc
    })
}

fn get_char_map(value: char) -> usize {
    let lowercase = String::from("abcdefghijklmnopqrstuvwxyz");
    let uppercase = lowercase.to_uppercase();

    match value.is_lowercase() {
        true => {
            return lowercase
                .char_indices()
                .find(|(_, v)| v.eq(&value))
                .expect("Failed to find lowercase")
                .0
                + 1;
        }
        false => {
            return uppercase
                .char_indices()
                .find(|(_, v)| v.eq(&value))
                .expect("Failed to find lowercase")
                .0
                + 27
        }
    }
}
