struct Increasing<TypeIter: Iterator> {
    inner: TypeIter,
    last: Option<TypeIter::Item>,
}

impl<TypeIter: Iterator> Iterator for Increasing<TypeIter>
where
    <TypeIter as Iterator>::Item: PartialOrd + Clone,
{
    type Item = TypeIter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.last {
            None => {
                self.last = self.inner.next();
                self.last.clone()
            }
            Some(_) => {
                while let Some(val) = self.inner.next() {
                    if val > self.last.clone().unwrap() {
                        return Some(val.clone());
                    }
                }
                None
            }
        }
    }
}

#[allow(dead_code)]
impl<I: Iterator> Increasing<I> {
    pub fn new<J>(iter: J) -> Self
    where
        J: IntoIterator<IntoIter = I>,
    {
        Increasing {
            inner: iter.into_iter(),
            last: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mandatory_test() {
        let mut iter = Increasing::new([0.4, 0.2, 0.1, 0.2, 0.4, 0.5, 0.4, 0.6]);
        assert_eq!(iter.next(), Some(0.4));
        assert_eq!(iter.next(), Some(0.5));
        assert_eq!(iter.next(), Some(0.6));
        assert_eq!(iter.next(), None);
    }
}
