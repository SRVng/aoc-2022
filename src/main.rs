use helper::Answers;

mod first;
mod helper;
mod second;
mod third;

fn main() {
    let answers: Vec<Answers> = Vec::from([
        Answers::AnsU32(first::get_answer(include_str!("first/input.txt"))),
        Answers::AnsU16(second::get_answer(include_str!("second/input.txt"))),
        Answers::AnsUsize(third::get_answer(include_str!("third/input.txt"))),
    ]);

    answers.into_iter().enumerate().for_each(|(index, value)| {
        value.print(index + 1);
    })
}
