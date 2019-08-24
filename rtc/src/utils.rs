/// Clamp function, see https://github.com/rust-lang/rust/issues/44095
/// Unstable as of writing this (23.08.19)
pub fn clamp<N: PartialOrd>(a: N, min: N, max: N) -> N {
    assert!(min <= max);
    let mut x = a;
    if x < min {
        x = min;
    } else if x > max {
        x = max;
    }
    x
}

/// Split lines that are over `max_length' long into multiple lines.
/// Breakes only at whitespace.
pub fn split_long_lines(max_length: usize, s: &String) -> Vec<String> {
    let mut length = 0;
    let mut line_buf = vec![];
    let mut buf = s.split_whitespace().fold(vec![], |mut buf, segment| {
        if length + segment.len() < max_length {
            length += segment.len() + 1;
            line_buf.push(segment);
        } else {
            buf.push(line_buf.join(" "));
            length = 0;
            line_buf.clear();
            length += segment.len();
            line_buf.push(segment);
        }
        buf
    });
    buf.push(line_buf.join(" "));
    buf
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn clamp_int() {
        assert_eq!(5, clamp(5, 0, 10));
        assert_eq!(1, clamp(5, 0, 1));
        assert_eq!(0, clamp(-5, 0, 1));
    }

    #[test]
    fn line_wrap() {
        let s = "123456789 123456789 123456789 123456789 123456789 123456789 123456789 abc defg 123456789 123456789 123456789 123456789 123456789 123456789 123456789 abc defg".to_string();
        assert_eq!(
            split_long_lines(70, &s),
            vec!(
                "123456789 123456789 123456789 123456789 123456789 123456789 123456789",
                "abc defg 123456789 123456789 123456789 123456789 123456789 123456789",
                "123456789 abc defg"
            )
        );
    }
}
