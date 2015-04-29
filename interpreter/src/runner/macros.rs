macro_rules! try_s(
    ($e:expr) => ( try!($e.map_err(|e| format!("{:?}", e))) )
);

macro_rules! file_to_string(
     ($p:expr) => {{
        use std::fs;
        use std::io::ErrorKind::NotFound;
        match fs::metadata(&$p) {
            Ok(..) => {},
            Err(ref e) if e.kind() == NotFound => return Err(format!("'{}' does not exist.",
                                                                     $p.to_string_lossy())),
            Err(e) => return Err(format!("Error getting metadata for '{}': {:?}",
                                         $p.to_string_lossy(), e))
        }

        let mut f = File::open($p).unwrap();
        let mut s = String::new();
        try_s!(f.read_to_string(&mut s));
        s
    }}
);

macro_rules! expect(
    ($iter:expr, $msg:expr) => (
        try!($iter.next().ok_or(format!("Failed to parse {}", $msg))).trim()
    )
);
