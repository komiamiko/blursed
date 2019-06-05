use nom::{named, escaped_transform, map, alt, call, tag};


named!(escape,
    map!(
        escaped_transform!(call!(alpha1), '\\',
            alt!(
                tag!("\\")  => { |_| &b"\\"[..] } |
                tag!("\"")  => { |_| &b"\""[..] } |
                tag!("n")   => { |_| &b"\n"[..] } |
                tag!("t")   => { |_| &b"\t"[..] } |
                tag!("r")   => { |_| &b"\r"[..] }
            )
        )
    )
);

pub fn parse_string_literal(s: &str) -> String {
    let bytes = s.as_bytes();
}


#[test]
fn basic_literal_test() {

}

