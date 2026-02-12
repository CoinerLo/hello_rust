#[derive(Debug, Eq, PartialEq)]
struct SimpleLog {
    entries: Vec<String>,
}

impl SimpleLog {
    fn new() -> Self {
        SimpleLog { entries: Vec::new() }
    }

    fn add(&mut self, message: impl Into<String>) {
        self.entries.push(message.into());
    }

    fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<A: Into<String>> FromIterator<A> for SimpleLog {
    fn from_iter<I: IntoIterator<Item = A>>(iter: I) -> Self {
        let mut log = SimpleLog::new();

        for i in iter {
            log.entries.push(i.into());
        }

        log
    }
}

struct SimpleLogIterator {
    entries: Vec<String>,
    front: usize,
    back: usize,
}

impl Iterator for SimpleLogIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }

        let index = self.front;
        self.front += 1;
        Some(std::mem::replace(&mut self.entries[index], String::new()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.back - self.front;
        (remaining, Some(remaining))
    }
}

impl DoubleEndedIterator for SimpleLogIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }

        self.back -= 1;
        let index = self.back;
        Some(std::mem::replace(&mut self.entries[index], String::new()))
    }
}

impl ExactSizeIterator for SimpleLogIterator {}

impl IntoIterator for SimpleLog {
    type Item = String;

    type IntoIter = SimpleLogIterator;

    fn into_iter(self) -> Self::IntoIter {
        let len = self.entries.len();
        SimpleLogIterator { entries: self.entries, front: 0, back: len }
    }
}

struct RefLogIterator<'a> {
    entries: &'a [String],
    front: usize,
    back: usize,
}

impl<'a> Iterator for RefLogIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }

        let index = self.front;
        self.front += 1;
        Some(&self.entries[index])
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.back - self.front;
        (remaining, Some(remaining))
    }
}

impl<'a> DoubleEndedIterator for RefLogIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }

        self.back -= 1;
        let index = self.back;
        Some(&self.entries[index])
    }
}

impl<'a> ExactSizeIterator for RefLogIterator<'a> {}

impl<'a> IntoIterator for &'a SimpleLog {
    type Item = &'a str;

    type IntoIter = RefLogIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let len = self.entries.len();
        RefLogIterator { entries: self.entries.as_slice(), front: 0, back: len }
    }
}

struct RefMutLogIterator<'a> {
    entries: &'a mut [String],
    front: usize,
    back: usize,
}

impl<'a> Iterator for RefMutLogIterator<'a> {
    type Item = &'a mut String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }

        let index = self.front;
        self.front += 1;

        let mut_ref: &'a mut String = unsafe {
            std::mem::transmute(&mut self.entries[index])
        };

        Some(mut_ref)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.back - self.front;
        (remaining, Some(remaining))
    }
}

impl<'a> DoubleEndedIterator for RefMutLogIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }

        self.back -= 1;
        let index = self.back;

        let mut_ref: &'a mut String = unsafe {
            std::mem::transmute(&mut self.entries[index])
        };

        Some(mut_ref)
    }
}

impl<'a> ExactSizeIterator for RefMutLogIterator<'a> {}

impl<'a> IntoIterator for &'a mut SimpleLog {
    type Item = &'a mut String;

    type IntoIter = RefMutLogIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let len = self.entries.len();
        RefMutLogIterator { entries: self.entries.as_mut_slice(), front: 0, back: len }
    }
}

fn main() {
    let mut log = SimpleLog::new();

    log.add("App started");
    log.add("User logged in");
    log.add("Error: connection timeout");

    for msg in &log {
        println!("→ {}", msg);
    }

    for msg in (&mut log).into_iter().rev() {
        println!("→ {}", msg);
    }

    let messages = vec![
        "App started".to_string(),
        "User logged in".to_string(),
        "Error: connection timeout".to_string(),
    ];

    assert_eq!(messages.into_iter().collect::<SimpleLog>(), log);
}
