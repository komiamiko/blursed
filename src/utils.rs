use nom::{named, escaped_transform, map, alt, call, tag, alpha};


fn to_s(i: Vec<u8>) -> String {
    String::from_utf8_lossy(&i).into_owned()
}

named!(escape < String >,
    map!(
        escaped_transform!(call!(alpha), '\\',
            alt!(
                tag!("\\")  => { |_| &b"\\"[..] } |
                tag!("\"")  => { |_| &b"\""[..] } |
                tag!("n")   => { |_| &b"\n"[..] } |
                tag!("t")   => { |_| &b"\t"[..] } |
                tag!("r")   => { |_| &b"\r"[..] }
            )
        ), to_s
    )
);

pub fn parse_string_literals(s: &str) -> String {
    let bytes = s.as_bytes();
    let out = escape(bytes).unwrap();
    out.1
}

#[cfg(test)]
mod tests {
    use super::parse_string_literals;

    const LITERAL_MAPS: &[(&str, &str)] = &[
        (r#"\n"#, "\n"),
        (r#"\\"#, "\\"),
        (r#"\t"#, "\t"),
        (r#"\r"#, "\r"),
    ];

    #[test]
    fn basic_literal_test() {
        LITERAL_MAPS.iter().for_each(|(r, p)|{
            let out = parse_string_literals(*r);
            let out_raw = out.as_str();
            assert_eq!(out_raw, *p);
        });
    }
}

