use qqbot_sdk::{CommonMessage, ReplyingMessage};
use qqbot_sdk::ReplyingMessage::Text;
use crate::context::fake_db::FakeDb;

pub struct MeCmd {
    pub(crate) db: FakeDb,
}

impl MeCmd {
    pub fn me(&self, msg: &dyn CommonMessage) -> ReplyingMessage {
        let profile = self.db.get_profile(msg.get_author_openid());
        Text(String::from(format!("You are {profile}.")))
    }
}