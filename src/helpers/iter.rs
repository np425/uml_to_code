use std::iter::Peekable;

pub trait ParserFuncs {
    fn expect_word(&mut self, word: &str) -> bool;
    fn expect_char(&mut self, chr: char) -> bool;
    fn expect_whitespace(&mut self);
    fn expect_newline(&mut self);
    fn next_word(&mut self) -> Option<String>;
    fn next_line(&mut self) -> Option<String>;
}

impl<Iter> ParserFuncs for Peekable<Iter>
    where Iter: Iterator<Item=char> + Clone {
    #[inline]
    fn expect_word(&mut self, word: &str) -> bool {
        let rollback = self.clone();

        self.expect_whitespace();

        match word.chars().all(|chr| Some(chr) == self.next()) {
            true => true,
            false => {
                *self = rollback;
                false
            }
        }
    }

    #[inline]
    fn expect_char(&mut self, chr: char) -> bool {
        self.expect_whitespace();
        self.next_if(|c| *c == chr).is_some()
    }

    #[inline]
    fn expect_whitespace(&mut self) {
        while self.next_if(|chr| chr.is_whitespace()).is_some() {}
    }

    fn expect_newline(&mut self) {
        while self.next_if(|chr| *chr == '\n').is_some() {}
    }

    #[inline]
    fn next_word(&mut self) -> Option<String> {
        let mut word = String::new();

        self.expect_whitespace();

        while let Some(chr) = self.next_if(|chr| chr.is_alphabetic() || chr.is_numeric()) {
            word.push(chr);
        }

        (word != "").then_some(word)
    }

    fn next_line(&mut self) -> Option<String> {
        let mut line = String::new();
        while let Some(chr) = self.next_if(|chr| *chr != '\n') {
            line.push(chr);
        }
        (line != "").then_some(line)
    }
}
