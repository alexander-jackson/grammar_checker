use std::convert::TryFrom;

#[derive(Debug)]
pub struct Production<'a> {
    pub output: Vec<&'a str>,
}

impl<'a> Production<'a> {
    pub fn new(output: Vec<&'a str>) -> Self {
        Self { output }
    }
}

impl<'a> TryFrom<&'a str> for Production<'a> {
    type Error = &'static str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(Production::new(value.split(' ').collect()))
    }
}
