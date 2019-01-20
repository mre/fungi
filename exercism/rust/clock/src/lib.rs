#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (mut hh, mut mm) = (hours, minutes);
        while mm < 0 {
            hh -= 1;
            mm += 60
        }
        hh += mm / 60;
        while hh < 0 {
            hh += 24
        }

        Clock {
            hours: hh % 24,
            minutes: mm % 60,
        }
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}
