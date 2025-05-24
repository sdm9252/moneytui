
use crate::type_game::LetterState::{Correct, Untyped, Wrong};



pub struct TypeGame {
    pub cursor: usize,
    pub words: Vec<WordState>,
}


///   invariants:
///   errors <= offset <= letters.len() - 1
///   only 20 errors allowed over the target word length
#[derive(Debug)]
pub struct WordState {
    pub letters: Vec<LetterState>,
    pub offset: usize,
    errors: usize
}

impl WordState {
    pub fn new(word_in: &str) -> WordState {
        let mut new_word_state_vec = Vec::new();
        for letter in word_in.chars() {
            new_word_state_vec.push(Untyped(letter))
        };

        WordState {
            letters: new_word_state_vec,
            offset: 0,
            errors: 0,
        }
    }

    pub fn is_complete(&self) -> bool {
        //println!("{:?}", self);
        self.errors == 0 &&
            self.offset == self.letters.len()
    }
}

#[derive(Debug)]
pub enum LetterState {
    Correct (char),
    Wrong (Option<char>, char), // (target, input)
    Untyped(char),
}

impl LetterState {
    pub fn is_untyped(&self) -> bool {
        match self {
            Correct(_) => false,
            Wrong(_, _) => false,
            Untyped(_) => true,
        }
    }
}

impl TypeGame {
    pub fn new(target: &str) -> TypeGame {
        let mut start_words = Vec::new();

        for word in target.split(' ') {
            start_words.push(WordState::new(word))
        }

        TypeGame {
            cursor: 0,
            words: start_words,
        }
    }

    pub fn push(&mut self, input: char) {
        assert!(self.cursor < self.words.len(),
                "word cursor past words length!");

        let current_word = self.words.get_mut(self.cursor).unwrap();

        assert!(current_word.offset <= current_word.letters.len(),
                "letter cursor past word length");

        assert!(current_word.errors <= current_word.offset,
                "more errors than letters typed in word");

        if input == ' ' {
            //some channel sending might happen here too
            if current_word.offset == 0 {
                return;
            }
            self.cursor += 1;
            return;
        }



        if current_word.offset == current_word.letters.len() - 1
        {
            //println!("test");

            if !current_word.letters[current_word.letters.len() -1].is_untyped() {
                //clearly never reaching here
                return;
            }
        }

        match current_word.letters.get_mut(current_word.offset) {
            Some(Untyped(letter)) => {
                if *letter == input {
                    current_word.letters[current_word.offset] = Correct(input);
                    current_word.offset += 1;
                } else {
                    current_word.letters[current_word.offset] = Wrong(
                        Some(*letter),
                        input
                    );
                    current_word.offset += 1;
                    current_word.errors += 1;
                }
            }
            None => {
                current_word.letters.push(Wrong(None, input));
                current_word.offset += 1;
                current_word.errors += 1;
            }
            _ => unreachable!()
        }
    }

    pub fn delete(&mut self) {
        assert!(self.cursor < self.words.len(),
                "word cursor past words length!");

        let current_word = self.words.get_mut(self.cursor).unwrap();

        assert!(current_word.offset <= current_word.letters.len(),
                "letter cursor past word length");

        assert!(current_word.errors <= current_word.offset,
                "more errors than letters typed in word");

 //       Theres more that needs doing
        if current_word.offset != 0 {
            match current_word.letters[current_word.offset - 1] {
                Correct(c) => {
                    current_word.letters[current_word.offset - 1] = Untyped(c);
                }
                Wrong(t_opt, _) => {
                    match t_opt {
                        Some(k) => {
                            current_word.letters[current_word.offset - 1] = Untyped(k)
                        }
                        None => {let _ = current_word.letters.pop();}
                    }
                    current_word.errors-=1;
                }
                Untyped(_) => {unreachable!()}
            }
            current_word.offset -= 1;
            return;
        }

        if self.cursor == 0 {
            return;
        }
        if self.words[self.cursor-1].is_complete() {
                return;
        }

        self.cursor -= 1;
    }

    //pub fn is_won(&self) -> bool { self.cursor == self.letters.len() && self.num_incorrect == 0 }
}