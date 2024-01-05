use std::error::Error;
use std::fmt::{Display, Formatter};
use uuid::Uuid;
use crate::services::authorizer::auth;
use crate::services::authorizer::auth::{AddressInfo, EducationInfo, WorkInfo};
use crate::services::authorizer::reward::Balance;

#[derive(Debug)]
struct Mail {
    address: String,
}

impl Display for Mail {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for Mail {}

impl Mail {
    fn new(address: String) -> Result<Self, Box<dyn Error>> {
        todo!()
    }
}

#[derive(Debug)]
pub struct User {
    id: Uuid,
    name: String,
    middle_name: Option<String>,
    second_name: Option<String>,
    mail: Mail,
    reward_balance: Balance,
    verified: bool,
    education_info: Option<EducationInfo>,
    address_info: Option<AddressInfo>,
    work_info: Option<WorkInfo>,
    auth_info: Option<auth::AuthInfo>
}
impl User {
    fn new(name: String) -> Self {
        User {
            id: Default::default(),
            name: "".to_string(),
            middle_name: None,
            second_name: None,
            mail: Mail { address: "".to_string() },
            reward_balance: Balance {},
            verified: false,
            education_info: None,
            address_info: None,
            work_info: None,
            auth_info: None,
        }
    }
}