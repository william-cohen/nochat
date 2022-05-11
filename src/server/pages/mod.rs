const CSS: &str = include_str!("styles.css");
const CHAT_PARTIAL: &str = include_str!("first.html");

pub fn render_chat_room() -> String {
    CHAT_PARTIAL.to_string()
}
