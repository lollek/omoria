#[cfg(not(test))]
mod globals;
#[cfg(not(test))]
mod interop;

#[derive(Debug, PartialEq)]
struct RestRequest {
    turns: i64,
    until_full: bool,
}

fn parse_rest(input: &[u8]) -> RestRequest {
    if input == b"*" {
        return RestRequest {
            turns: 20,
            until_full: true,
        };
    }
    let start = input
        .iter()
        .take_while(|byte| matches!(byte, b' ' | b'\t'..=b'\r'))
        .count();
    let input = &input[start..];
    let sign_length = usize::from(matches!(input.first(), Some(b'+' | b'-')));
    let digits = input[sign_length..]
        .iter()
        .take_while(|byte| byte.is_ascii_digit())
        .count();
    let turns = std::str::from_utf8(&input[..sign_length + digits])
        .ok()
        .and_then(|number| number.parse().ok())
        .unwrap_or(0);
    RestRequest {
        turns,
        until_full: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_star_requests_twenty_turns_until_full() {
        assert_eq!(
            parse_rest(b"*"),
            RestRequest {
                turns: 20,
                until_full: true,
            }
        );
    }

    #[test]
    fn decimal_prefix_matches_scanf() {
        for (input, turns) in [
            ("42", 42),
            ("12junk", 12),
            (" \t\n\r\x0b\x0c+17end", 17),
            ("-9tail", -9),
            ("0012", 12),
            ("0x12", 0),
            ("9999999999", 9_999_999_999),
        ] {
            assert_eq!(
                parse_rest(input.as_bytes()),
                RestRequest {
                    turns,
                    until_full: false,
                },
                "input: {input:?}"
            );
        }
    }

    #[test]
    fn zero_and_invalid_input_do_not_request_rest() {
        for input in ["0", "-0", "junk", "", " *", "* ", "+", "--2", "+ 2"] {
            assert_eq!(
                parse_rest(input.as_bytes()),
                RestRequest {
                    turns: 0,
                    until_full: false,
                },
                "input: {input:?}"
            );
        }
    }
}
