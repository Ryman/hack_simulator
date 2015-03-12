#[macro_export] macro_rules! check(
    (
        $modn:ident
        for { $($name:ident $k:expr => $v:expr),+ }
        do |$x:ident, $y:ident| $b:block
    ) => {
        #[cfg(test)]
        mod $modn {
            use hack_assembler::*;
            $(
                #[test]
                fn $name() {
                    let f = |$x, $y| $b;
                    f($k, $v)
                }
            )+
        }
    };
);
