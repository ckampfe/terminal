use ratatui::{
    style::{Color, Modifier, Style},
    text::{Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use rustler::ResourceArc;

pub(crate) struct ParagraphResource<'a>(ratatui::widgets::Paragraph<'a>);

impl<'a: 'static> rustler::Resource for ParagraphResource<'a> {}

// fn load(env: rustler::Env, _: rustler::Term) -> bool {
//     env.register::<ParagraphResource>().unwrap();
//     true
// }

#[rustler::nif(schedule = "DirtyIo", name = "paragraph_new")]
fn new(
    block: rustler::ResourceArc<crate::block::BlockResource<'static>>,
    text: String,
) -> ResourceArc<ParagraphResource<'_>> {
    let mut block = block.0.lock().unwrap();

    let paragraph = Paragraph::new(Text::from(text.to_string()))
        .block(block.take().unwrap())
        .wrap(Wrap { trim: false });

    rustler::ResourceArc::new(ParagraphResource(paragraph))
}

#[rustler::nif(schedule = "DirtyIo", name = "paragraph_render")]
fn render(
    terminal: ResourceArc<crate::terminal::TerminalResource>,
    text: String,
    chunks: ResourceArc<crate::terminal::ChunksResource>,
    index: usize,
) {
    // let widget = paragraph.0;
    // let chunk = chunks.0;

    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Info",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ));

    let paragraph = Paragraph::new(Text::from(text))
        .block(block)
        .wrap(Wrap { trim: false });

    let mut t = terminal.terminal.lock().unwrap();
    let mut frame = t.get_frame();
    frame.render_widget(paragraph, chunks.0[index]);
}
