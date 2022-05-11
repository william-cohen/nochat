#[derive(Clone)]
pub struct Message {
    content: String,
    username: String
}

impl Message {
    pub fn new(content: String, username: String) -> Self {
        Message {
            content,
            username
        }
    }

    pub fn get_content<'a>(&'a self) -> &'a str {
        &self.content
    } 

    pub fn get_user<'a>(&'a self) -> &'a str {
        &self.username
    }
}