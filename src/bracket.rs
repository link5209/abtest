// https://exercism.io/tracks/rust/exercises/matching-brackets/solutions/103e946f00924574bbd5c016b49d1037
pub struct Brackets { is_balanced: bool }

impl Brackets {
    pub fn from(s: &str) -> Self {
        let mut open_brackets_stack = Vec::new();

        for c in s.chars() {
            match c {
                // opening a bracket/brace is always fine
                '{' | '(' | '[' => open_brackets_stack.push(c),
                // closing is only fine when it matches the most recent opened
                '}' | ')' | ']' => {
                    if let Some(prev) = open_brackets_stack.pop() {
                        match (prev, c) {
                            // closing correct pair, all fine (already popped)
                            ('{', '}') | ('(', ')') | ('[', ']') => continue,
                            // closing a wrong pair, bail
                            _ => return Self { is_balanced: false }
                        }
                    } else { // closing without any open pair, bail
                        return Self { is_balanced: false };
                    }
                },
                // ignore anything that's not a bracket, for this exercise
                _ => continue
            }
        }

        // only balanced if we didn't bail early *and* nothing left on the stack
        Self { is_balanced: open_brackets_stack.len() == 0 }
    }

    pub fn are_balanced(&self) -> bool {
        self.is_balanced
    }
}
