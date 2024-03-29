pub struct SplitConsecutiveWithIndices<'a> {
    haystack: &'a str,
    chars: &'a [char],
    start: usize,
}

impl<'a> Iterator for SplitConsecutiveWithIndices<'a> {
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<(usize, &'a str)> {
        let total = self.haystack.len();
        if self.start == total {
            return None;
        }

        let tail = &self.haystack[self.start..];

        match tail.find(self.chars) {
            Some(start) => {
                let end = self.start
                    + start
                    + 'find_end: {
                        let mut last = start;
                        for (index, c) in tail[start..].chars().enumerate() {
                            if !self.chars.contains(&c) {
                                break 'find_end index;
                            }
                            last = index + c.len_utf8();
                        }
                        last
                    };
                let start = self.start + start;

                if self.start == start {
                    //consecutive delim matches, skip to next match
                    let start = end;
                    let end = match &self.haystack[start..].find(self.chars) {
                        Some(i) => start + i,
                        None => self.haystack.len(),
                    };
                    let s = &self.haystack[start..end];
                    self.start = end;
                    if s.is_empty() {
                        None
                    } else {
                        Some((start, s))
                    }
                } else {
                    let s = &self.haystack[self.start..start];
                    let index = self.start;
                    self.start = start;
                    Some((index, s))
                }
            }
            None => {
                let s = &self.haystack[self.start..];
                let index = self.start;
                self.start = self.haystack.len();
                Some((index, s))
            }
        }
    }
}

pub trait SplitConsecutiveIndicesExt:
    ::std::ops::Index<::std::ops::RangeFull, Output = str>
{
    fn split_consecutive_with_indices<'p>(
        &'p self,
        chars: &'p [char],
    ) -> SplitConsecutiveWithIndices<'p> {
        SplitConsecutiveWithIndices {
            haystack: &self[..],
            chars,
            start: 0,
        }
    }
}

impl SplitConsecutiveIndicesExt for str {}
