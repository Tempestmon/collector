use crate::services::examinator::survey::Survey;

trait Examine {
    fn answer();
    fn fill_info();
}

impl Examine for Survey {
    fn answer() {
        todo!()
    }

    fn fill_info() {
        todo!()
    }
}
