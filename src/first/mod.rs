use crate::helper::Answer;

pub fn get_answer(input: &str) -> Answer<u32, u32> {
    let mut data: Vec<Vec<u32>> = vec![vec![]];

    input.split('\n').fold(0, |mut acc, value| {
        if !value.is_empty() {
            data[acc].push(value.parse().expect("Invalid input string"));
        } else {
            data.push(vec![]);
            acc += 1;
        }
        acc
    });

    let mut sum: Vec<u32> = data
        .into_iter()
        .map(|chunk| {
            chunk.into_iter().fold(0, |mut acc, value| {
                acc += value;
                acc
            })
        })
        .collect::<Vec<u32>>();
    sum.sort_by(|a, b| b.cmp(a));

    let first_answer = sum[0];
    let second_answer = vec![sum[0], sum[1], sum[2]]
        .into_iter()
        .fold(0, |mut acc, value| {
            acc += value;
            acc
        });

    Answer {
        first_answer,
        second_answer,
    }
}

#[cfg(test)]
mod test {
    use super::{get_answer, Answer};

    #[test]
    pub fn test_get_answer() {
        let input = include_str!("example.txt");
        let Answer {
            first_answer,
            second_answer,
        } = get_answer(input);

        assert_eq!(first_answer, 24000);
        assert_eq!(second_answer, 45000);
    }
}
