pub trait ExtractIntegers<U> {
    fn uints(&self) -> Vec<U>;
}

macro_rules! impl_extract_integers {
    ($unsigned:ty) => {
        impl ExtractIntegers<$unsigned> for &str {
            fn uints(&self) -> Vec<$unsigned> {
                // let r = Regex::new(r"(\d+)").unwrap();
                // let mut out = Vec::new();

                // for captures in r.captures_iter(self) {
                //     let capture = captures.get(0).unwrap();
                //     let i = capture.as_str().parse().unwrap();

                //     out.push(i)
                // }

                // out
                let mut out = Vec::new();
                let mut current = None;

                for c in self.chars() {
                    match c.to_digit(10).map(|n| n as $unsigned) {
                        Some(d) => match current.as_mut() {
                            Some(n) => *n = (*n * 10) + d,
                            None => current = Some(d),
                        },

                        None => match current.take() {
                            Some(n) => out.push(n),
                            None => {}
                        },
                    }
                }

                if let Some(n) = current.take() {
                    out.push(n);
                }

                out
            }
        }
    };
}

impl_extract_integers!(u8);
impl_extract_integers!(u16);
impl_extract_integers!(u32);
impl_extract_integers!(u64);
impl_extract_integers!(usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uints() {
        let haystack = "(1,2,3) hello 456 world?78-90\n1337.42";
        let uints: Vec<u32> = haystack.uints();

        assert_eq!(uints, vec![1, 2, 3, 456, 78, 90, 1337, 42])
    }
}
