pub struct Parser<'a> {
    remaining: &'a str,
    current: &'a str
}

#[derive(Debug, PartialEq)]
pub enum Command { A, C, L }

impl<'a> Parser<'a> {
    pub fn new(input: &str) -> Parser {
        Parser { remaining: input, current: "" }
    }

    pub fn has_more_commands(&self) -> bool {
        self.remaining.lines().any(|line| {
            let line = line.trim();
            !(line.is_empty() || line.starts_with("//"))
        })
    }

    pub fn advance(&mut self) {
        self.current = "";
        while self.current.is_empty() {
            let mut line = self.remaining.splitn(2, '\n');
            let current = line.next().unwrap();

            // Strip trailing comments
            // FIXME: Should be "//" but the trait impl is missing in std \o/
            self.current = current.splitn(2, '/').next().unwrap().trim();
            self.remaining = line.next().unwrap_or("");
        }
    }

    pub fn command_type(&self) -> Command {
        match self.current.chars().nth(0) {
            Some('@') => Command::A,
            Some('(') => Command::L,
            _ => Command::C
        }
    }

    pub fn symbol(&self) -> &'a str {
        match self.command_type() {
            Command::A => &self.current[1..],
            Command::L => self.current.trim_matches(&['(', ')'][..]),
            _ => wrong_command_type("symbol")
        }
    }
    pub fn dest(&self) -> &'a str {
        if self.command_type() != Command::C { wrong_command_type("dest") }

        self.current.find('=')
                    .map(|idx| &self.current[0..idx])
                    .unwrap_or("")
    }

    pub fn comp(&self) -> &'a str {
        if self.command_type() != Command::C { wrong_command_type("comp") }

        let idx = self.current.find(';')
                              .unwrap_or(self.current.len());
        self.current[0..idx]
            .rsplitn(2, '=')
            .next()
            .unwrap_or("")
    }
    pub fn jump(&self) -> &'a str {
        if self.command_type() != Command::C { wrong_command_type("jump") }

        self.current.find(';')
                    .map(|idx| &self.current[idx+1..])
                    .unwrap_or("")
    }

    // This could probably be an iterator but there are tricky lifetimes
    // to maintain the api presented in the book
    pub fn each_advance<F, E>(&mut self, mut f: F) -> Result<(), E>
            where F: FnMut(&mut Parser<'a>) -> Option<E> {
        while self.has_more_commands() {
            self.advance();

            if let Some(error) = f(self) { return Err(error) }
        }

        Ok(())
    }
}

fn wrong_command_type(fname: &str) -> ! {
    panic!("{} unsupported for current command_type", fname)
}
