mod home;
mod view_item;

pub use home::Home;
use term_lib::command::CmdList;
pub use view_item::ViewItem;

use rebrickable_database_api::RebrickableDB;

use crate::{Cmd, DB};

use std::io::Write;

pub trait Mode<RDB: RebrickableDB, W: Write> {
    fn get_possible_cmds(&self) -> CmdList<Cmd>;

    fn handle_cmd(
        self: Box<Self>,
        cmd: Cmd,
        db: &DB,
        rebrickable_db: &RDB,
    ) -> Box<dyn Mode<RDB, W>>;

    fn render(&self, w: &mut W) -> term_lib::Result<()>;
}
