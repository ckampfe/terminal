use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
    widgets::Borders,
};
use rustler::ResourceArc;

pub(crate) struct BlockResource<'a>(pub std::sync::Mutex<Option<ratatui::widgets::Block<'a>>>);

impl rustler::Resource for BlockResource<'static> {
    const IMPLEMENTS_DESTRUCTOR: bool = false;
    const IMPLEMENTS_DOWN: bool = false;
}

#[rustler::nif(schedule = "DirtyIo")]
fn block_new() -> rustler::ResourceArc<BlockResource<'static>> {
    rustler::ResourceArc::new(BlockResource(std::sync::Mutex::new(Some(
        ratatui::widgets::Block::default(),
    ))))
}

#[rustler::nif(schedule = "DirtyIo")]
fn block_borders(
    block: rustler::ResourceArc<BlockResource<'static>>,
) -> rustler::ResourceArc<BlockResource<'static>> {
    {
        let mut lock = block
            .0
            .lock()
            .expect("must be able to take in blockborders");
        let inner = lock.take().unwrap();
        *lock = Some(inner.borders(Borders::ALL))
    }

    block
}

#[rustler::nif(schedule = "DirtyIo")]
fn block_title(
    block: ResourceArc<BlockResource<'static>>,
    title: &str,
) -> ResourceArc<BlockResource<'static>> {
    {
        let mut lock = block.0.lock().unwrap();
        let inner = lock.take().expect("must be able to take in blocktitle");
        *lock = Some(
            inner.title(Span::styled(
                title.to_owned(),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
        )
    }

    block
}
