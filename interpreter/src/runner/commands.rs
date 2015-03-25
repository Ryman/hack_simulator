use self::Command::*;

#[derive(Debug)]
pub enum Command<'a> {
    Load(&'a str),
    OutputFile(&'a str),
    CompareTo(&'a str),
    OutputList(Vec<&'a str>),
    Set(&'a str, i16),
    Repeat(u16, Vec<Command<'a>>),
    TickTock,
    Output
}

impl<'a> Command<'a> {
    fn parse(s: &'a str) -> Result<Command<'a>, String> {
        debug!("Parsing Command: '{}'", s);

        let mut parts = s.split(|c| c == ' ' || c == '\t' || c == '\n' || c == '\r');
        let cmd = match expect!(parts, "name") {
            "load" => Load(expect!(parts, "filename for load")),
            "output-file" => OutputFile(expect!(parts, "filename for output-file")),
            "compare-to" => CompareTo(expect!(parts, "filename for compare-to")),
            "output-list" => OutputList(parts.map(|s| s.trim())
                                             .filter(|s| !s.is_empty())
                                             .collect()),
            "set" => {
                let location = expect!(parts, "location for set");
                let raw_value = expect!(parts, "value for set");
                let value = try_s!(raw_value.parse());
                Set(location, value)
            }
            "repeat" => unreachable!(), // Handle in Commands Iterator
            "ticktock" => TickTock,
            "output" => Output,
            cmd => return Err(format!("Unexpected command: '{}'", cmd))
        };

        Ok(cmd)
    }
}

impl<'a> Commands<'a> {
    pub fn new(s: &str) -> Commands {
        Commands { remaining: s }
    }
}

pub struct Commands<'a> {
    remaining: &'a str,
}

impl<'a> Iterator for Commands<'a> {
    type Item = Result<Command<'a>, String>;

    fn next(&mut self) -> Option<Result<Command<'a>, String>> {
        self.remaining = self.remaining.trim_left();
        if self.remaining.len() == 0 { return None }

        // Skip commented lines
        if self.remaining.starts_with("//") {
            return self.remaining.find('\n').and_then(|idx| {
                    self.remaining = &self.remaining[idx+1..];
                    self.next()
            });
        }

        // Special case for repeat
        if self.remaining.starts_with("repeat") {
            return parse_repeat(&mut self.remaining)
        }

        let seperators: &[_] = &[',', ';']; // Coercion fail.
        return self.remaining.find(seperators).map(|idx| {
            let cmd = Command::parse(&self.remaining[0..idx]);
            self.remaining = &self.remaining[idx+1..];
            cmd
        });

        fn parse_repeat<'a>(remaining: &mut &'a str) -> Option<Result<Command<'a>, String>> {
            let start = match remaining.find('{') {
                Some(idx) => idx,
                None => return Some(Err("Missing '{' after 'repeat'".to_string()))
            };

            let iterations = &remaining["repeat".len()..start].trim();
            let iterations = match iterations.parse() {
                Ok(i) => i,
                Err(e) => return Some(Err(format!("Failed to parse iteration count \
                                                   for 'repeat': {} - '{}'",
                                                   e, iterations)))
            };

            // FIXME: This won't handle nested repeat commands
            let end = match remaining.find('}') {
                Some(idx) => idx,
                None => return Some(Err("Missing '}' after 'repeat'".to_string()))
            };

            let mut commands = vec![];
            for command in Commands::new(&remaining[start+1..end]) {
                match command {
                    Ok(cmd) => commands.push(cmd),
                    Err(e) => return Some(Err(e))
                }
            }

            *remaining = &remaining[end+1..];
            Some(Ok(Repeat(iterations, commands)))
        }
    }
}
