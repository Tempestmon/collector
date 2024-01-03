use crate::common::user::{User};

#[derive(Debug)]
pub struct AuthInfo {
    token: String,
}

trait Authorize {
    fn authorize() -> bool;
    fn verify() -> bool;
    fn register() -> bool;
}

trait FillInfo {
    fn fill_education_info() -> Self;
    fn fill_address_info() -> Self;
    fn fill_work_info() -> Self;
}

impl FillInfo for User {

    fn fill_education_info() -> Self {
        todo!()
    }

    fn fill_address_info() -> Self {
        todo!()
    }

    fn fill_work_info() -> Self {
        todo!()
    }
}


#[derive(Debug)]
pub struct EducationInfo {

}

#[derive(Debug)]

pub struct AddressInfo {

}

#[derive(Debug)]
pub struct WorkInfo {

}