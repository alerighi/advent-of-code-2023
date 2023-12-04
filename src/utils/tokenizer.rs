pub trait Tokenizer {
    fn peek_char(&self) -> Option<char>;
    fn next_char(&mut self) -> Option<char>;
    fn read_until(&mut self, until: char) -> String;
    fn next_digit(&mut self) -> String;
    fn skip_whitespace(&mut self);
}

pub struct StringTokenizer {
    position: usize,
    chars: Vec<char>,
}

impl From<&str> for StringTokenizer {
    fn from(string: &str) -> Self {
        StringTokenizer {
            position: 0,
            chars: string.chars().collect(),
        }
    }
}

impl Tokenizer for StringTokenizer {
    fn peek_char(&self) -> Option<char> {
        if self.position < self.chars.len() {
            Some(self.chars[self.position])
        } else {
            None
        }
    }

    fn next_char(&mut self) -> Option<char> {
        if self.position < self.chars.len() {
            let result = self.chars[self.position];
            self.position += 1;
            Some(result)
        } else {
            None
        }
    }

    fn read_until(&mut self, until: char) -> String {
        let mut result = String::new();
        loop {
            match self.next_char() {
                None => break result,
                Some(ch) => {
                    if until == ch {
                        break result
                    } 
                    result.push(ch);
                }
            }
        }
    }

    fn next_digit(&mut self) -> String {
        let mut result = String::new();
        
        while self.peek_char().is_some_and(|c| !c.is_ascii_digit()) {
            self.next_char();
        }

        while self.peek_char().is_some_and(|c| c.is_ascii_digit()) {
            result.push(self.next_char().unwrap());
        }

        result
    }

    fn skip_whitespace(&mut self) {
        while self.peek_char().is_some_and(|c| c.is_ascii_whitespace()) {
            self.next_char();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::tokenizer::Tokenizer;

    #[test]
    fn get_next_char() {
        let mut tokenizer = StringTokenizer::from("ab");
        assert_eq!(tokenizer.next_char(), Some('a'));
        assert_eq!(tokenizer.next_char(), Some('b'));
        assert_eq!(tokenizer.next_char(), None);
        assert_eq!(tokenizer.next_char(), None);
    }

    #[test]
    fn read_until() {
        let mut tokenizer: StringTokenizer = StringTokenizer::from("abc;;test");
        assert_eq!(tokenizer.read_until(';'), "abc");
        assert_eq!(tokenizer.read_until(';'), "");
        assert_eq!(tokenizer.read_until(';'), "test");
        assert_eq!(tokenizer.read_until(';'), "");
    }

    #[test]
    fn read_digit() {
        let mut tokenizer: StringTokenizer = StringTokenizer::from("aabbc123 67");
        assert_eq!(tokenizer.next_digit(), "123");
        assert_eq!(tokenizer.peek_char(), Some(' '));
        assert_eq!(tokenizer.next_digit(), "67");
    }
}
