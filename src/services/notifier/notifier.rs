use uuid::Uuid;
use crate::services::examinator::survey::Survey;
use crate::services::authorizer::user::User;

struct Subscription {
    id: Uuid,
    user: User,
    surveys: Vec<Survey>,
}

impl Subscription {
    fn new() -> Self {
        todo!()
    }

    fn subscribe(survey: Survey) {
        todo!()
    }

    fn notify() {
        todo!()
    }
}