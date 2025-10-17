use {
    clap::ValueEnum,
    core::str::FromStr,
    crate::tape::*
    };


/* Value visualisation mode */
#[derive(Clone, Copy, Default, ValueEnum)]
pub enum DisplayMode {
    ASCII,
    #[default]
    Numeric
    }


/* Function for quick checking whether ascii can be printed */
pub fn is_ascii_printable<T>(value: T) -> bool
where T: TapeCell {
    /* Try casting into an u8, only then check if it fits within the ranges */
    match value.to_u8() {
        Some(byte) =>
            matches!(byte, b'\t' ..= b'\n' | b' ' ..= b'~'),
        _ => false
        }
    }

/* Function for parsing written input buffer */
pub fn parse_line<T>(buf: &str) -> Result<T, <T as FromStr>::Err>
where T: TapeCell {
    let value = buf.trim();

    /* Try parsing the buffer as a char literal */
    if let &[b'\'', byte, b'\''] = value.as_bytes() {
        return Ok(T::from(byte));
        }

    /* Pare as a normal integer */
    value.parse()
    }


#[cfg(test)]
mod test {
    use crate::utils::*;

    #[test]
    fn ascii_printable_numbers() {
        let numbers = ('0' ..= '9')
            .map(u32::from)
            .all(is_ascii_printable);

        assert!(numbers);
        }

    #[test]
    fn ascii_printable_letters() {
        let uppercase = ('A' ..= 'Z')
            .map(u32::from)
            .all(is_ascii_printable);
        let lowercase = ('a' ..= 'z')
            .map(u32::from)
            .all(is_ascii_printable);

        assert!(uppercase);
        assert!(lowercase);
        }

    #[test]
    fn ascii_printable_signs() {
        let signs = ('!' ..= '/')
            .chain(':' ..= '@')
            .chain('[' ..= '`')
            .chain('{' ..= '~')
            .map(u32::from)
            .all(is_ascii_printable);

        assert!(signs);
        }

    #[test]
    fn ascii_printable_whitespaces() {
        let whitespaces = ['\t', '\n', ' '].into_iter()
            .map(u32::from)
            .all(is_ascii_printable);

        assert!(whitespaces);
        }

    #[test]
    fn ascii_printable_unprintable() {
        let unprintable = ! ('\0' ..).take(9)
            .map(u32::from)
            .any(is_ascii_printable);

        assert!(unprintable);
        }

    #[test]
    fn parse_letters() {
        assert_eq!(parse_line("'A'"), Ok(b'A'));
        assert_eq!(parse_line("'Z'"), Ok(b'Z'));
        assert_eq!(parse_line("'a'"), Ok(b'a'));
        assert_eq!(parse_line("'z'"), Ok(b'z'));
        }

    #[test]
    fn parse_numbers() {
        assert_eq!(parse_line("'0'"), Ok(b'0'));
        assert_eq!(parse_line("'1'"), Ok(b'1'));
        assert_eq!(parse_line("'9'"), Ok(b'9'));
        }

    #[test]
    fn parse_signs() {
        assert_eq!(parse_line("'!'"), Ok(b'!'));
        assert_eq!(parse_line("'/'"), Ok(b'/'));
        assert_eq!(parse_line("':'"), Ok(b':'));
        assert_eq!(parse_line("'@'"), Ok(b'@'));
        assert_eq!(parse_line("'['"), Ok(b'['));
        assert_eq!(parse_line("'`'"), Ok(b'`'));
        assert_eq!(parse_line("'{'"), Ok(b'{'));
        assert_eq!(parse_line("'~'"), Ok(b'~'));
        }

    #[test]
    fn parse_other() {
        assert_eq!(parse_line("'\t'"), Ok(b'\t'));
        assert_eq!(parse_line("'\n'"), Ok(b'\n'));
        assert_eq!(parse_line("' '"), Ok(b' '));
        assert_eq!(parse_line("'\0'"), Ok(b'\0'));
        }

    #[test]
    fn parse_raw_u8() {
        assert_eq!(parse_line("0"), Ok(0u8));
        assert_eq!(parse_line("9"), Ok(9u8));
        assert_eq!(parse_line("10"), Ok(10u8));
        assert_eq!(parse_line("99"), Ok(99u8));
        assert_eq!(parse_line("100"), Ok(100u8));
        assert_eq!(parse_line("199"), Ok(199u8));
        assert_eq!(parse_line("200"), Ok(200u8));
        assert_eq!(parse_line("249"), Ok(249u8));
        assert_eq!(parse_line("250"), Ok(250u8));
        assert_eq!(parse_line("255"), Ok(255u8));
        assert!(parse_line::<u8>("256").is_err());
        assert!(parse_line::<u8>("300").is_err());
        }

    #[test]
    fn parse_raw_u16() {
        assert_eq!(parse_line("0"), Ok(0u16));
        assert_eq!(parse_line("9"), Ok(9u16));
        assert_eq!(parse_line("10"), Ok(10u16));
        assert_eq!(parse_line("200"), Ok(200u16));
        assert_eq!(parse_line("3000"), Ok(3000u16));
        assert_eq!(parse_line("40000"), Ok(40000u16));
        assert_eq!(parse_line("50000"), Ok(50000u16));
        assert_eq!(parse_line("60000"), Ok(60000u16));
        assert_eq!(parse_line("65535"), Ok(65535u16));
        assert!(parse_line::<u16>("65536").is_err());
        assert!(parse_line::<u16>("70000").is_err());
    }

    #[test]
    fn parse_raw_u32() {
        assert_eq!(parse_line("0"), Ok(0u32));
        assert_eq!(parse_line("9"), Ok(9u32));
        assert_eq!(parse_line("10"), Ok(10u32));
        assert_eq!(parse_line("200"), Ok(200u32));
        assert_eq!(parse_line("3000"), Ok(3000u32));
        assert_eq!(parse_line("40000"), Ok(40000u32));
        assert_eq!(parse_line("500000"), Ok(500000u32));
        assert_eq!(parse_line("6000000"), Ok(6000000u32));
        assert_eq!(parse_line("70000000"), Ok(70000000u32));
        assert_eq!(parse_line("800000000"), Ok(800000000u32));
        assert_eq!(parse_line("900000000"), Ok(900000000u32));
        assert_eq!(parse_line("4000000000"), Ok(4000000000u32));
        assert_eq!(parse_line("4294967295"), Ok(4294967295u32));
        assert!(parse_line::<u32>("4294967296").is_err());
        assert!(parse_line::<u32>("5000000000").is_err());
    }

    #[test]
    fn parse_padded() {
        assert_eq!(parse_line("      'A'      "), Ok(65u8));
        assert_eq!(parse_line("'A'            "), Ok(65u8));
        assert_eq!(parse_line("            'A'"), Ok(65u8));
        assert_eq!(parse_line("      1      "), Ok(1u8));
        assert_eq!(parse_line("1            "), Ok(1u8));
        assert_eq!(parse_line("            1"), Ok(1u8));
        assert_eq!(parse_line("01"), Ok(1u8));
        assert_eq!(parse_line("001"), Ok(1u8));
        assert_eq!(parse_line("0000000"), Ok(0u8));
        }

    #[test]
    fn parse_incorrect() {
        assert!(parse_line::<u8>("-0").is_err());
        assert!(parse_line::<u8>("-1").is_err());
        assert!(parse_line::<u8>("'      a      '").is_err());
        assert!(parse_line::<u8>("'a            '").is_err());
        assert!(parse_line::<u8>("'            a'").is_err());
        }
    }