use crate::helper::Answer;

pub fn get_answer(input: &str) -> Answer<u16, u16> {
    let pairs: Vec<&str> = input.split('\n').collect();
    let assignments: Vec<Vec<&str>> = pairs
        .into_iter()
        .map(|pairs| pairs.split(',').collect::<Vec<&str>>())
        .collect();
    let coverage: Vec<Vec<Vec<u16>>> = assignments
        .into_iter()
        .map(|p| {
            p.into_iter()
                .map(|i| {
                    i.split('-')
                        .map(|s| s.parse::<u16>().expect("Failed to parse str"))
                        .collect::<Vec<u16>>()
                })
                .collect::<Vec<Vec<u16>>>()
        })
        .collect();

    coverage.into_iter().fold(
        Answer {
            first_answer: 0,
            second_answer: 0,
        },
        |mut acc, val| {
            pub struct Parameters {
                first_start: u16,
                first_end: u16,
                second_start: u16,
                second_end: u16,
            }

            fn get_parameter(val: &[Vec<u16>]) -> Parameters {
                let first_start = val[0][0];
                let first_end = val[0][1];
                let second_start = val[1][0];
                let second_end = val[1][1];

                Parameters {
                    first_start,
                    first_end,
                    second_start,
                    second_end,
                }
            }

            fn check_first_condition<'a>(
                value: &'a Vec<Vec<u16>>,
                mut accumulator: &'a mut Answer<u16, u16>,
            ) -> &'a Answer<u16, u16> {
                let Parameters {
                    first_start,
                    first_end,
                    second_start,
                    second_end,
                } = get_parameter(value);
                if first_start <= second_start && first_end >= second_end {
                    accumulator.first_answer += 1;
                    return accumulator;
                }

                if first_start >= second_start && first_end <= second_end {
                    accumulator.first_answer += 1;
                    return accumulator;
                }

                accumulator
            }

            fn check_second_condition<'a>(
                value: &'a Vec<Vec<u16>>,
                mut accumulator: &'a mut Answer<u16, u16>,
            ) -> &'a Answer<u16, u16> {
                let Parameters {
                    first_start,
                    first_end,
                    second_start,
                    second_end,
                } = get_parameter(value);
                if first_end <= second_end && first_end >= second_start {
                    accumulator.second_answer += 1;
                    return accumulator;
                }

                if first_end >= second_end && first_start <= second_end {
                    accumulator.second_answer += 1;
                    return accumulator;
                }

                accumulator
            }

            check_first_condition(&val, &mut acc);
            check_second_condition(&val, &mut acc);

            acc
        },
    )
}

#[cfg(test)]
mod tests {
    use super::{get_answer, Answer};

    #[test]
    pub fn test_get_answer() {
        let input = include_str!("example.txt");
        let Answer {
            first_answer,
            second_answer,
        } = get_answer(input);

        assert_eq!(first_answer, 2);
        assert_eq!(second_answer, 4);
    }
}
