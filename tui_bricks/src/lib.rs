extern crate log;

mod cmd;
mod data;
pub mod error;
mod io;
mod mode;
mod state;

#[cfg(not(debug_assertions))]
mod config;

#[cfg(not(debug_assertions))]
fn get_user_db_path() -> error::Result<std::path::PathBuf> {
    use config::Config;
    use figment::{
        Figment,
        providers::{Format, Serialized, Yaml},
    };

    let mut config_path = io::get_config_dir();
    config_path.push("config.yml");

    let config: Config = Figment::from(Serialized::defaults(Config::default()))
        .merge(Yaml::file(config_path))
        .extract()
        .unwrap();

    Ok(config.get_db_path())
}

pub fn run() -> error::Result<()> {
    env_logger::init();
    let mut w = std::io::stdout();

    #[cfg(not(debug_assertions))]
    let db_path = get_user_db_path()?;

    #[cfg(debug_assertions)]
    let db_path = std::path::PathBuf::new().join("test_db.yml");

    let mut state = state::State::new(db_path)?;

    term_lib::init(&mut w)?;

    loop {
        match state.wait_for_cmd(&mut w) {
            Ok(()) => {}
            Err(error::Error::TermError(term_lib::Error::Quit)) => {
                term_lib::quit(&mut w)?;
                return Ok(());
            }
            Err(e) => {
                term_lib::quit(&mut w)?;
                return Err(e);
            }
        }
    }
}
