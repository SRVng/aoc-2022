use crate::helper::Answer;

pub fn get_answer(input: &str) -> Answer<usize, usize> {
    let rucksack: Vec<String> = input.split('\n').map(|s| s.to_string()).collect();

    let compartments: Vec<Vec<String>> = rucksack
        .clone()
        .into_iter()
        .map(|r| {
            let mut first = r.clone();
            let rest = first.split_off(r.capacity() / 2);
            vec![first, rest]
        })
        .collect();

    let first_answer = compartments
        .into_iter()
        .map(|v| intersect(v[0].clone(), v[1].clone(), None))
        .map(get_char_map)
        .sum::<usize>();

    let group: Vec<Vec<String>> = rucksack
        .chunks(3)
        .map(|v| v.to_vec())
        .collect::<Vec<Vec<String>>>();

    let second_answer = group
        .into_iter()
        .map(|v| intersect(v[0].clone(), v[1].clone(), Some(v[2].clone())))
        .map(get_char_map)
        .sum::<usize>();

    Answer {
        first_answer,
        second_answer,
    }
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

#[cfg(test)]
mod test {
    use super::{get_answer, Answer};

    #[test]
    fn test_get_answer() {
        let input = include_str!("example.txt");
        let Answer {
            first_answer,
            second_answer,
        } = get_answer(input);
        assert_eq!(first_answer, 157);
        assert_eq!(second_answer, 70);
    }
}
