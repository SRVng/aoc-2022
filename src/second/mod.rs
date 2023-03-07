type Score = u16;

pub fn get_answer() {
    let input: &str = include_str!("input.txt");

    let data: Vec<Vec<char>> = input
        .split('\n')
        .map(|value| {
            value
                .split(' ')
                .map(|c| c.chars().next().expect("Empty string"))
                .collect::<Vec<char>>()
        })
        .collect();

    dbg!(data
        .clone()
        .into_iter()
        .map(|v| get_score(v[0], v[1]))
        .sum::<Score>());
    dbg!(data
        .into_iter()
        .map(|v| get_score_second(v[0], v[1]))
        .sum::<Score>());
}

// A X Rock
// B Y Paper
// C Z Scissor

fn get_score(opponent: char, us: char) -> Score {
    fn get_shape_score(shape: char) -> Score {
        match shape {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => 0,
        }
    }

    match opponent {
        'A' => {
            let mut score = get_shape_score(us);
            match us {
                'X' => score += 3,
                'Y' => score += 6,
                _ => (),
            }
            score
        }
        'B' => {
            let mut score = get_shape_score(us);
            match us {
                'Y' => score += 3,
                'Z' => score += 6,
                _ => (),
            }
            score
        }
        'C' => {
            let mut score = get_shape_score(us);
            match us {
                'Z' => score += 3,
                'X' => score += 6,
                _ => (),
            }
            score
        }
        _ => 0,
    }
}

// A Rock
// B Paper
// C Scissor
// X Lose
// Y Draw
// Z Win

fn get_score_second(opponent: char, us: char) -> Score {
    fn get_result_score(result: char) -> Score {
        match result {
            'Y' => 3,
            'Z' => 6,
            _ => 0,
        }
    }
    let mut score = get_result_score(us);
    match opponent {
        'A' => match us {
            'X' => {
                score += 3; // Scissor
            }
            'Y' => {
                score += 1; // Rock
            }
            'Z' => {
                score += 2; // Paper
            }
            _ => (),
        },
        'B' => match us {
            'X' => {
                score += 1; // Rock
            }
            'Y' => {
                score += 2; // Paper
            }
            'Z' => {
                score += 3; // Scissor
            }
            _ => (),
        },
        'C' => match us {
            'X' => {
                score += 2; // Paper
            }
            'Y' => {
                score += 3; // Scissor
            }
            'Z' => {
                score += 1; // Rock
            }
            _ => (),
        },
        _ => (),
    }
    score
}
