pub mod commands;
pub mod context;

use crate::commands::me::MeCmd;
use crate::context::fake_db::FakeDb;
use config::Config;
use qqbot_sdk::{AppConfig, CommonMessage, Context, run_application};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder()
        .add_source(::config::File::with_name("config.toml").required(false))
        .build();
    let config: AppConfig = config?.try_deserialize()?;

    let db = FakeDb {};
    let me_cmd = MeCmd { db };
    let config = config
        .with_context(Context::new(FakeDb {}))
        .with_command("/me", move |msg: &dyn CommonMessage| me_cmd.me(msg));

    run_application(config).await?;
    Ok(())
}
