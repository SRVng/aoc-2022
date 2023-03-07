pub fn get_answer() {
    let input = include_str!("input.txt");

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
    println!("First answer: {}", first_answer);
    println!("Second answer: {}", second_answer);
}
