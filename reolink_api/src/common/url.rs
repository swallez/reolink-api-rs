use std::sync::LazyLock;

/// Tweaks the query-string of a URL by de-encoding some url-encoded characters that the
/// Reolink API (at least Home Hub) doesn't accept in their encoded form, notably '/' (cannot be
/// `%2F`) and '+' used to represent space (has to be `%20`) which are present in the URL for the
/// download endpoint.
///
/// Returns `None` if the query string can be used as is.
///
/// The argument must be a valid url-encoded query string, this function may panic otherwise.
///
/// Example: `"foo%2Fbar+baz"` will be converted to `Some("foo/bar%20baz")`.
///
pub(crate) fn tweak_url(url: &mut reqwest::Url) {
    if let Some(qs) = url.query() {
        if let Some(new_qs) = tweak_query_string(qs) {
            url.set_query(Some(&new_qs));
        }
    }
}

fn tweak_query_string(qs: &str) -> Option<String> {
    let mut iter = PercentEncodingIter::new(qs);
    let mut maybe_step = iter.next();

    // Fast path: no percent-encodings
    if let Some(Step::Unencoded(chunk)) = maybe_step {
        if chunk.len() == qs.len() {
            return None;
        }
    }

    let allowed = *ALLOWED;
    let mut buf = Vec::<u8>::with_capacity(qs.len());
    while let Some(step) = maybe_step {
        match step {
            Step::Unencoded(section) => {
                buf.extend(section)
            },
            Step::Plus => {
                buf.extend(b"%20")
            },
            Step::Encoded(section) => {
                let char = as_u8(section[1])*16 + as_u8(section[2]);
                if is_allowed(allowed, char) {
                    buf.push(char);
                } else {
                    buf.extend(section[0..3].iter());
                }
            },
        }
        maybe_step = iter.next();
    }

    // SAFETY: input is a valid string in which we replace some characters with ascii chars which are valid UTF-8.
    let result = unsafe { String::from_utf8_unchecked(buf) };
    Some(result)
}

/// Bit mask of ascii characters (< 128) we want to allow that are normally percent-encoded.
static ALLOWED: LazyLock<u128> = LazyLock::new(|| {
    let allowed = br#"!"$&'()*,-./:;<>?@[]^_`{}~"#;
    let mut result: u128 = 0;
    for c in allowed {
        result |= 1 << *c as u32;
    }
    result
});

#[inline]
fn is_allowed(mask: u128, c: u8) -> bool {
    // c is lower than 128 and the corresponding mask bit is set
    (c & 0x80 == 0) && (mask & (1 << c) != 0)
}

/// Values returned by `PercentEncodingIter` to iterate on chunks of encoded/non-encoded
/// sections of a query string.
enum Step<'a> {
    /// A percent-encoded character (3 characters)
    Encoded(&'a [u8]),
    /// An unencoded sequence
    Unencoded(&'a [u8]),
    /// A plus sign that encodes a space
    Plus,
}

struct PercentEncodingIter<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl <'a> PercentEncodingIter<'a> {
    fn new(qs: &'a str) -> PercentEncodingIter<'a> {
        PercentEncodingIter { bytes: qs.as_bytes(), pos: 0 }
    }
}

impl <'a> Iterator for PercentEncodingIter<'a> {
    type Item = Step<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.bytes.len() {
            return None;
        }

        match self.bytes[self.pos] {
            b'%' => {
                let result = Step::Encoded(&self.bytes[self.pos..self.pos + 3]);
                self.pos += 3;
                Some(result)
            },
            b'+' => {
                self.pos += 1;
                Some(Step::Plus)
            },
            _ => {
                // Find the beginning of the next encoded section, or else end of input
                let next_pos = self.bytes.iter()
                    // position_from doesn't exist (even in itertools)
                    .skip(self.pos).position(|b| *b == b'%' || *b == b'+').map(|p| p + self.pos)
                    .unwrap_or_else(|| self.bytes.len());
                let result = Step::Unencoded(&self.bytes[self.pos..next_pos]);
                self.pos = next_pos;
                Some(result)
            },
        }
    }
}

#[inline]
fn as_u8(v: u8) -> u8 {
    match v {
        b'0'..=b'9' => v - b'0',
        b'A'..=b'F' => v - b'A' + 10,
        b'a'..=b'f' => v - b'a' + 10,
        _ => panic!("invalid character '{}'", v as char),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qs_tweak() {
        assert!(tweak_query_string("foobar").is_none());
        assert_eq!(Some("foo/bar"), tweak_query_string("foo%2Fbar").as_deref());
        assert_eq!(Some("foo/bar%20baz"), tweak_query_string("foo%2Fbar%20baz").as_deref());
        assert_eq!(Some("%20foo/bar%20baz"), tweak_query_string("%20foo%2Fbar+baz").as_deref());
        assert_eq!(Some("/foo/bar%20baz"), tweak_query_string("%2Ffoo%2Fbar%20baz").as_deref());
        assert_eq!(Some("/fox/bar%20baz/"), tweak_query_string("%2Ffox%2Fbar%20baz%2F").as_deref());
        assert_eq!(Some("%20fox/bar%20baz%20"), tweak_query_string("%20fox%2Fbar+baz+").as_deref());
        assert_eq!(Some("%20fox//bar%20baz%20"), tweak_query_string("%20fox%2F%2Fbar%20baz%20").as_deref());
        assert_eq!(Some("%20fox/%20bar%20baz%20"), tweak_query_string("%20fox%2F%20bar%20baz%20").as_deref());
    }

    #[test]
    fn qs_tweak_non_ascii() -> anyhow::Result<()> {
        let client = reqwest::Client::new();
        let req = client.get("http://example.com/foo")
            .query(&[("foo", "bar🎉/déjà vu")])
            .build()?;

        let qs = req.url().query().unwrap();
        // Non-ascii UTF-8 only contains bytes > 128. '/' and ' ' are encoded
        assert_eq!("foo=bar%F0%9F%8E%89%2Fd%C3%A9j%C3%A0+vu", qs);
        // Non-ascii UTF-8 kept as is, '/' is decoded and ' ' is reencoded as '%20'
        assert_eq!("foo=bar%F0%9F%8E%89/d%C3%A9j%C3%A0%20vu", tweak_query_string(qs).unwrap());
        Ok(())
    }
}
