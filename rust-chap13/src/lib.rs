pub struct Counter {
    pub count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        return Counter { count: 0 };
    }
}

impl Iterator for CounterIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter.count < 5 {
            self.counter.count += 1;
            return Some(self.counter.count);
        } else {
            return None;
        }
    }
}

impl IntoIterator for Counter {
    type Item = u32;

    type IntoIter = CounterIterator;

    fn into_iter(self) -> Self::IntoIter {
        return CounterIterator {
            counter: self,
            index: 0,
        };
    }
}

pub struct CounterIterator {
    pub counter: Counter,
    pub index: u32,
}

#[test]
fn calling_next() {
    let mut counter: CounterIterator = Counter::new().into_iter();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn calling_iter() {
    let counter: Counter = Counter::new();

    let mut count: u32 = 0;

    for num in counter.into_iter() {
        assert_eq!(num, count);
        count += 1;
    }
}
