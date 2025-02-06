// допустимо иметь единственного владельца
// разрешены неизменяемые или изменяемые владения данными, проверяемые во время выполнения а не компиляции
// внутренняя изменяемость

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTrackeer<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTrackeer<'a, T>
where T: Messenger
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTrackeer<'a, T> {
        LimitTrackeer {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your qouta!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        send_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                send_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.send_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTrackeer::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.send_messages.borrow().len(), 1);
    }
}
