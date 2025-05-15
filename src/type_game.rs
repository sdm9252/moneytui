use std::collections::VecDeque;
use crate::type_game::LetterState::{Untyped, Wrong};

pub struct TypeGame {
    pub cursor: usize,
    pub letters: VecDeque<LetterState>,
    num_incorrect: usize
}
#[derive(Debug)]
pub enum LetterState {
    Correct (char),
    Wrong (Option<char>, char), // (target, input)
    Untyped(char),
}

impl TypeGame {
    pub fn new(target: &str) -> TypeGame {
        let mut start_letters = VecDeque::new();
        for nxt_char in target.chars()
            { start_letters.push_back(Untyped(nxt_char)) };

        //println!("{:?}", start_letters);

        TypeGame {
            cursor: 0,
            letters: start_letters,
            num_incorrect: 0
        }
    }

    pub fn push(&mut self, input: char) {
        match self.letters.get(self.cursor) {
            Some(Untyped(target_char)) => {
                *(self.letters.get_mut(self.cursor).unwrap())  =
                    if *target_char == input {
                        LetterState::Correct(input)
                    } else {
                        self.num_incorrect += 1;
                        Wrong(Some(*target_char), input)
                    }
            }
            None => { self.letters.push_back(Wrong(None, input)); self.num_incorrect+=1 },
            _ => unreachable!(),
        }
        self.cursor +=1;
    }

    pub fn delete(&mut self) {
        if self.cursor == 0 {
            return
        }
        match self.letters.get(self.cursor - 1) {
            Some(Wrong(Some(replace), _)) => {
                self.num_incorrect -= 1;
                *(self.letters.get_mut(self.cursor - 1).unwrap()) = Untyped(*replace)
            },
            Some(LetterState::Correct(replace)) => {
                *(self.letters.get_mut(self.cursor - 1).unwrap()) = Untyped(*replace)
            },
            Some(Wrong(None, _)) => {
                self.letters.pop_back();
                self.num_incorrect -= 1;
            },
            _ => unreachable!()
        }
        self.cursor -= 1;
    }

    //pub fn is_won(&self) -> bool { self.cursor == self.letters.len() && self.num_incorrect == 0 }
}