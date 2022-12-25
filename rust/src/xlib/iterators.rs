#[derive(Debug)]
struct ExpolodeToZeroIterator {
    start: isize,
    offset: isize,
    direction: bool, // true for higher / false for lower ( to 0 )
}

impl ExpolodeToZeroIterator {
    fn new(start: isize) -> ExpolodeToZeroIterator {
        ExpolodeToZeroIterator {
            start,
            offset: 1,
            direction: true,
        }
    }
}

impl Iterator for ExpolodeToZeroIterator {
    type Item = isize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.start {
            None
        } else {
            if self.direction {
                let result = self.start + self.offset;
                self.direction = false;
                Some(result)
            } else {
                let result = self.start - self.offset;
                self.direction = true;
                self.offset += 1;
                Some(result)
            }
        }
    }
}
