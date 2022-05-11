const CHAT_PARTIAL: &str = include_str!("room.html");
const LOGIN: &str = include_str!("login.html");

pub fn render_chat_room(username: &String) -> String {
    CHAT_PARTIAL.to_string().replace(r"{{username}}", username)
}

pub fn render_login_page() -> String {
    LOGIN.to_string()
}
