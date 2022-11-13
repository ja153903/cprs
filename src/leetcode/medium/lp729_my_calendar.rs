#![allow(dead_code)]

struct MyCalendar {
    events: Vec<(i32, i32)>,
}

impl MyCalendar {
    fn new() -> Self {
        Self { events: Vec::new() }
    }

    // TODO: Solve this problem later
    fn book(&mut self, _start: i32, _end: i32) -> bool {
        // do binary search to find the appropriate place to place the interval
        false
    }
}

#[cfg(test)]
mod tests {
    // use super::MyCalendar;

    #[test]
    pub fn should_pass_sample_case() {
        // let mut cal = MyCalendar::new();

        // assert_eq!(cal.book(10, 20), true);
        // assert_eq!(cal.book(15, 25), false);
        // assert_eq!(cal.book(20, 30), true);
    }
}
