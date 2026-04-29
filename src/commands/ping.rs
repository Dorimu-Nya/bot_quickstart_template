use qqbot_sdk::ReplyingMessage;
use qqbot_sdk::ReplyingMessage::Text;
use qqbot_sdk_macros::command;

#[command("/ping")]
fn ping() -> ReplyingMessage {
    Text(String::from("Pong!"))
}
