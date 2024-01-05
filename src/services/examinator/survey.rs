use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

struct Choice {

}

struct Question {
    id: Uuid,
    question: String,
    choice: Choice,
}

enum Parameter {

}

pub struct Survey {
    id: Uuid,
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