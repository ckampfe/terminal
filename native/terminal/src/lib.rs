use rustler::{Env, Term};
use terminal::TerminalResource;

pub(crate) mod block;
pub(crate) mod paragraph;
pub(crate) mod terminal;

fn load(env: Env, _term: Term) -> bool {
    env.register::<crate::block::BlockResource>().unwrap();
    env.register::<crate::paragraph::ParagraphResource>()
        .unwrap();
    env.register::<TerminalResource>().unwrap();
    env.register::<terminal::ChunksResource>().unwrap();
    true
}

rustler::init!("Elixir.Terminal.Native", load = load);
