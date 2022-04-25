#[derive(Clone)]
pub struct Message {
    content: String,
    username: String
}

impl Message {
    pub fn new(content: &str) -> Self {
        Message {
            content: content.to_owned(),
            username: "Dummy".to_owned()
        }
    }

    pub fn get_content<'a>(&'a self) -> &'a str {
        &self.content
    } 

    pub fn get_user<'a>(&'a self) -> &'a str {
        &self.username
    }
}