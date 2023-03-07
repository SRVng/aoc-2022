use std::fmt::Display;

pub enum Answers {
    AnsU16(Answer<u16, u16>),
    AnsU32(Answer<u32, u32>),
    AnsUsize(Answer<usize, usize>),
}

impl Answers {
    pub fn print(&self, index: usize) {
        match self {
            Answers::AnsU16(ans) => ans.print(index),
            Answers::AnsU32(ans) => ans.print(index),
            Answers::AnsUsize(ans) => ans.print(index),
        }
    }
}

pub struct Answer<T, K> {
    pub first_answer: T,
    pub second_answer: K,
}

impl<T: Display, K: Display> Answer<T, K> {
    pub fn print(&self, index: usize) {
        let Self {
            first_answer,
            second_answer,
        } = self;
        println!(
            "Question: {}, First answer={} Second answer={}",
            index, first_answer, second_answer
        );
    }
}
