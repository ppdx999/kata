#[derive(Debug, PartialEq)]
pub struct Schema {}

impl Schema {
    pub fn from_text(_text: &str) -> Result<Self, String> {
        Ok(Schema {})
    }

    pub fn validate(&self, _line: String) -> Result<(), String> {
        Ok(())
    }
}
