use std::collections::HashMap;
use chrono::{DateTime, Utc};

struct Choice {

}

struct Question {
    question: String,
    choice: Choice,
}

enum Parameter {

}

pub struct Survey {
    questions: Vec<Question>,
    parameters: HashMap<Parameter, bool>,
    creation_date: DateTime<Utc>,
    expiration_date: Option<DateTime<Utc>>,
}

struct Template {

}

trait Creation {
    fn create() -> Box<Self> {
        todo!()
    }
}

impl Creation for Choice {

}

impl Creation for Question {

}

impl Creation for Survey {

}

impl Survey {
    fn share() {
        todo!()
    }

    fn change_privacy() {
        todo!()
    }

    fn set_time_constraint() {
        todo!()
    }
}