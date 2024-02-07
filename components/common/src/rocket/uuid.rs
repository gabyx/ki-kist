use rocket::request::FromParam;
use uuid::Uuid;

// We can only implement a trait from an own type, so
// we simply wrap it.
pub struct WrappedUuid(Uuid);

impl WrappedUuid {
    pub fn unwrap(&self) -> Uuid {
        return self.0;
    }
}

impl<'a> FromParam<'a> for WrappedUuid {
    type Error = uuid::Error;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        return Uuid::try_parse(param).map(|res| WrappedUuid(res));
    }
}
