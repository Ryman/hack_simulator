macro_rules! try_s(
    ($e:expr) => ( try!($e.map_err(|e| format!("{:?}", e))) )
);

macro_rules! file_to_string(
     ($p:expr) => {{
        if !$p.exists() {
            return Err(format!("{} does not exist.", $p.to_string_lossy()))
        }

        let mut f = File::open($p).unwrap();
        let mut s = String::new();
        try_s!(f.read_to_string(&mut s));
        s
    }}
);

macro_rules! expect(
    ($iter:expr, $msg:expr) => (
        try!($iter.next().ok_or(format!("Failed to parse {}", $msg)))
    )
);
