pub struct Parser {
    pub input: String,
    pub current_ix: usize,
}

type Error = String;

impl Parser {
    pub fn new(input: String) -> Self {
        Parser {
            input,
            current_ix: 0,
        }
    }

    pub fn eof(&self) -> bool {
        self.current_ix >= self.input.len()
    }

    pub fn current_text(&self) -> &str {
        &self.input[self.current_ix..]
    }

    pub fn skip_whitespace(&mut self) {
        self.skip(|c| c.is_whitespace())
    }

    pub fn skip_eol(&mut self) {
        let mut count = 0;
        let mut current_text = self.current_text().chars();

        // Find next linebreak
        while let Some(c) = current_text.next() {
            count += 1;
            if c == '\n' || c == '\r' {
                break;
            }
        }

        // Step over linebreak
        while let Some(c) = current_text.next() {
            if c == '\n' || c == '\r' {
                count += 1;
            } else {
                break;
            }
        }

        self.current_ix += count;
    }

    pub fn skip_one(&mut self) {
        self.current_ix += 1;
    }

    pub fn skip_char(&mut self, ch: char) {
        self.skip(move |c| c == ch)
    }

    pub fn skip<P>(&mut self, f: P)
    where
        P: Fn(char) -> bool,
    {
        let mut count = 0;
        for c in self.current_text().chars() {
            let mat = f(c);
            if mat {
                count += 1;
            } else {
                break;
            }
        }

        self.current_ix += count;
    }

    /// Does NOT move the cursor
    fn inner_parse<P>(&mut self, f: P) -> Option<Vec<char>>
    where
        P: Fn(char) -> bool,
    {
        let result = self
            .current_text()
            .chars()
            .take_while(move |c| f(*c))
            .collect::<Vec<char>>();

        if result.len() == 0 {
            return None;
        }

        return Some(result);
    }

    pub fn parse_i64(&mut self) -> Option<i64> {
        let result = self.inner_parse(|c| c == '-' || c.is_numeric())?;

        let len = result.len();

        let result = result.iter().collect::<String>();

        let result = result.parse::<i64>().ok()?;

        self.current_ix += len;

        return Some(result);
    }

    pub fn parse_u64(&mut self) -> Option<u64> {
        let result = self.inner_parse(|c| c.is_numeric())?;

        let len = result.len();

        let result = result.iter().collect::<String>();

        let result = result.parse::<u64>().ok()?;

        self.current_ix += len;

        return Some(result);
    }

    pub fn parse_token(&mut self, token: &str) -> Option<String> {
        let current_text = self.current_text();
        if current_text.len() < token.len() {
            return None;
        }

        if !token.chars().eq(current_text.chars().take(token.len())) {
            return None;
        }

        self.current_ix += token.len();

        return Some(token.to_string());
    }

    pub fn find<T>(&mut self, ff: &dyn Fn(&mut Parser) -> Option<T>) -> Option<T> {
        let start_ix = self.current_ix;
        let mut min_res = ff(self);

        while min_res.is_none() {
            self.skip_one();
            min_res = ff(self);
        }

        if min_res.is_none() {
            self.current_ix = start_ix;
        }

        return min_res;
    }

    pub fn parse_word1(&mut self) -> Option<String> {
        let result = self
            .current_text()
            .chars()
            .take_while(|c| !c.is_whitespace())
            .collect::<String>();

        if result.len() == 0 {
            return None;
        }

        self.current_ix += result.len();

        return Some(result);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let mut parser = Parser::new("screen".to_owned());
        assert_eq!(parser.parse_token("screen"), Some("screen".to_owned()));
    }

    #[test]
    fn parse_u64() {
        let mut parser = Parser::new("64".to_owned());
        assert_eq!(parser.parse_u64(), Some(64));
    }

    #[test]
    fn parse_u64_fail() {
        let mut parser = Parser::new("-64".to_owned());
        assert_eq!(parser.parse_u64(), None);
    }

    #[test]
    fn parse_i64() {
        let mut parser = Parser::new("64".to_owned());
        assert_eq!(parser.parse_i64(), Some(64));
    }

    #[test]
    fn parse_i64_with_space() {
        let mut parser = Parser::new("64 ".to_owned());
        assert_eq!(parser.parse_i64(), Some(64));
    }

    #[test]
    fn parse_i64_negative() {
        let mut parser = Parser::new("-64".to_owned());
        assert_eq!(parser.parse_i64(), Some(-64));
    }

    #[test]
    fn parse_i64_negative_fail() {
        let mut parser = Parser::new("-64-64".to_owned());
        assert_eq!(parser.parse_i64(), None);
    }

    #[test]
    fn test_parse_token() {
        let mut parser = Parser::new("foobar".to_owned());
        assert_eq!(parser.parse_token("foobar"), Some("foobar".to_string()));
        assert_eq!(parser.eof(), true);
    }

    #[test]
    fn test_parse_token_part() {
        let mut parser = Parser::new("foobar".to_owned());
        assert_eq!(parser.parse_token("foo"), Some("foo".to_string()));
        assert_eq!(parser.eof(), false);
    }

    #[test]
    fn test_parse_token_no_match() {
        let mut parser = Parser::new("foobar".to_owned());
        assert_eq!(parser.parse_token("bar"), None);
    }

    #[test]
    fn parse_eol() {
        let mut parser = Parser::new(
            "-64-64
and another"
                .to_owned(),
        );
        parser.skip_eol();
        assert_eq!(parser.current_text(), "and another");
    }
}
