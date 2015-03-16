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
            let mut line = self.remaining.splitn(1, '\n');
            let current = line.next().unwrap();

            // Strip trailing comments
            // FIXME: Should be "//" but the trait impl is missing in std \o/
            self.current = current.splitn(1, '/').next().unwrap().trim();
            self.remaining = line.next().unwrap_or("");
        }
    }

    pub fn command_type(&self) -> Command {
        match self.current.char_at(0) {
            '@' => Command::A,
            '(' => Command::L,
            _ => Command::C
        }
    }

    pub fn symbol(&self) -> &str {
        match self.command_type() {
            Command::A => &self.current[1..],
            Command::L => self.current.trim_matches(&['(', ')'][..]),
            _ => wrong_command_type("symbol")
        }
    }
    pub fn dest(&self) -> &str {
        if self.command_type() != Command::C { wrong_command_type("dest") }

        self.current.find('=')
                    .map(|idx| &self.current[0..idx])
                    .unwrap_or("")
    }

    pub fn comp(&self) -> &str {
        if self.command_type() != Command::C { wrong_command_type("comp") }

        let idx = self.current.find(';')
                              .unwrap_or(self.current.len());
        self.current[0..idx]
            .rsplitn(1, '=')
            .next()
            .unwrap_or("")
    }
    pub fn jump(&self) -> &str {
        if self.command_type() != Command::C { wrong_command_type("jump") }

        self.current.find(';')
                    .map(|idx| &self.current[idx+1..])
                    .unwrap_or("")
    }
}

fn wrong_command_type(fname: &str) -> ! {
    panic!("{} unsupported for current command_type", fname)
}
