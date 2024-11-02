use ratatui::layout::{Direction, Layout, Rect};
use ratatui::prelude::{Backend, CrosstermBackend};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Span, Text};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Terminal;
use rustler::{Decoder, Env, NifResult, Resource, ResourceArc, Term};
use std::collections::HashMap;
use std::sync::Mutex;

mod atoms {
    rustler::atoms! {
        ok,

        active,
        passive,

        event,
        tick,

        focus_gained,
        focus_lost,
        paste,
        resize,
        press,
        repeat,
        release,

        todo,

        control,
        shift,
        alt,
        sooper,
        hyper,
        meta,
        none,
        char,
        enter,
        delete,
        backspace,
        left,
        right,
        up,
        down,
        home,
        insert,
        end,
        page_up,
        page_down,
        tab,
        back_tab,
        f,
        null,
        esc,
        caps_lock,
        scroll_lock,
        print_screen,
        num_lock,
        pause,
        menu,
        begin,


        keycode,
        code,
        modifiers,
        kind,
        state,

        percentage,

    }
}

pub(crate) struct TerminalResource {
    pub terminal: Mutex<Terminal<CrosstermBackend<std::io::Stdout>>>,
}

impl Resource for TerminalResource {
    const IMPLEMENTS_DESTRUCTOR: bool = true;

    const IMPLEMENTS_DOWN: bool = false;

    fn destructor(self, _env: Env<'_>) {
        let mut terminal = self.terminal.lock().unwrap();
        crossterm::terminal::disable_raw_mode().unwrap();
        crossterm::execute!(
            terminal.backend_mut(),
            crossterm::terminal::LeaveAlternateScreen
        )
        .unwrap();
        terminal.show_cursor().unwrap();
    }

    fn down<'a>(&'a self, _env: Env<'a>, _pid: rustler::LocalPid, _monitor: rustler::Monitor) {}
}

struct NakedTerminalResource {
    pub terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl Resource for NakedTerminalResource {}

macro_rules! nif_error {
    ($term:expr) => {
        rustler::Error::Term(Box::new($term.to_string()))
    };
}

#[derive(PartialEq)]
enum Mode {
    Active,
    Passive,
}

impl Decoder<'_> for Mode {
    fn decode(term: Term<'_>) -> NifResult<Self> {
        let term: rustler::Atom = term.decode()?;

        if term == atoms::active() {
            Ok(Mode::Active)
        } else if term == atoms::passive() {
            Ok(Mode::Passive)
        } else {
            Err(nif_error!("mode must be `:active` or `:passive`"))
        }
    }
}

#[rustler::nif(schedule = "DirtyIo")]
fn terminal_new(
    env: rustler::Env,
    tick_rate: u64,
    mode: Mode,
) -> NifResult<(rustler::Atom, ResourceArc<TerminalResource>)> {
    crossterm::terminal::enable_raw_mode().map_err(|e| nif_error!(e))?;

    let mut stdout = std::io::stdout();

    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)
        .map_err(|e| nif_error!(e))?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend).map_err(|e| nif_error!(e))?;

    terminal.hide_cursor().map_err(|e| nif_error!(e))?;

    terminal.clear().map_err(|e| nif_error!(e))?;

    let tick_rate = std::time::Duration::from_millis(tick_rate);

    if mode == Mode::Active {
        let caller_pid = env.pid();

        rustler::spawn::<rustler::ThreadSpawner, _>(env, move |env| {
            let mut last_tick = std::time::Instant::now();
            loop {
                // poll for tick rate duration, if no events, sent tick event.
                if crossterm::event::poll(tick_rate - last_tick.elapsed())
                    .expect("Unable to poll for Crossterm event")
                {
                    if let crossterm::event::Event::Key(key) =
                        crossterm::event::read().expect("Unable to read Crossterm event")
                    {
                        // event_tx
                        //     .send(Event::Input(key))
                        //     .expect("Unable to send Crossterm Key input event");
                        let _ = env.send(&caller_pid, (atoms::event(), KeyEvent::from(key)));
                    }
                }
                if last_tick.elapsed() >= tick_rate {
                    // event_tx.send(Event::Tick).expect("Unable to send tick");
                    let _ = env.send(&caller_pid, (atoms::event(), atoms::tick()));
                    last_tick = std::time::Instant::now();
                }
            }
        });
    }

    Ok((
        atoms::ok(),
        ResourceArc::new(TerminalResource {
            terminal: Mutex::new(terminal),
        }),
    ))
}

#[rustler::nif(name = "terminal_event_available?", schedule = "DirtyIo")]
fn is_event_available(milliseconds: u64) -> NifResult<(rustler::Atom, bool)> {
    Ok((
        atoms::ok(),
        crossterm::event::poll(std::time::Duration::from_millis(milliseconds))
            .map_err(|e| nif_error!(e))?,
    ))
}

struct Event(crossterm::event::Event);

impl From<crossterm::event::Event> for Event {
    fn from(value: crossterm::event::Event) -> Self {
        Self(value)
    }
}

impl rustler::Encoder for Event {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        match self.0 {
            crossterm::event::Event::FocusGained => atoms::focus_gained().to_term(env),
            crossterm::event::Event::FocusLost => atoms::focus_lost().to_term(env),
            crossterm::event::Event::Key(key_event) => {
                let ke: KeyEvent = key_event.into();
                ke.encode(env)
            }
            crossterm::event::Event::Mouse(_mouse_event) => todo!(),
            crossterm::event::Event::Paste(ref paste) => (atoms::paste(), paste).encode(env),
            crossterm::event::Event::Resize(columns, rows) => {
                (atoms::resize(), columns, rows).encode(env)
            }
        }
    }
}

struct KeyEvent {
    code: KeyCode,
    modifiers: KeyModifiers,
    kind: KeyEventKind,
    state: KeyEventState,
}

// struct KeyEvent(crossterm::event::KeyEvent);

// impl From<crossterm::event::KeyEvent> for KeyEvent {
//     fn from(value: crossterm::event::KeyEvent) -> Self {
//         Self(value)
//     }
// }

impl From<crossterm::event::KeyEvent> for KeyEvent {
    fn from(value: crossterm::event::KeyEvent) -> Self {
        KeyEvent {
            code: value.code.into(),
            modifiers: value.modifiers.into(),
            kind: value.kind.into(),

            state: value.state.into(),
        }
    }
}

impl rustler::Encoder for KeyEvent {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let mut hm = HashMap::new();
        hm.insert(atoms::code().encode(env), self.code.encode(env));
        hm.insert(atoms::modifiers().encode(env), self.modifiers.encode(env));
        hm.insert(atoms::kind().encode(env), self.kind.encode(env));
        hm.insert(atoms::state().encode(env), self.state.encode(env));
        hm.encode(env)
    }
}

struct KeyCode(crossterm::event::KeyCode);

impl From<crossterm::event::KeyCode> for KeyCode {
    fn from(value: crossterm::event::KeyCode) -> Self {
        Self(value)
    }
}

impl rustler::Encoder for KeyCode {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        match self.0 {
            crossterm::event::KeyCode::Backspace => {
                (atoms::keycode(), atoms::backspace()).encode(env)
            }
            crossterm::event::KeyCode::Enter => (atoms::keycode(), atoms::enter()).encode(env),
            crossterm::event::KeyCode::Left => (atoms::keycode(), atoms::left()).encode(env),
            crossterm::event::KeyCode::Right => (atoms::keycode(), atoms::right()).encode(env),
            crossterm::event::KeyCode::Up => (atoms::keycode(), atoms::up()).encode(env),
            crossterm::event::KeyCode::Down => (atoms::keycode(), atoms::down()).encode(env),
            crossterm::event::KeyCode::Home => (atoms::keycode(), atoms::home()).encode(env),
            crossterm::event::KeyCode::End => (atoms::keycode(), atoms::end()).encode(env),
            crossterm::event::KeyCode::PageUp => (atoms::keycode(), atoms::page_up()).encode(env),
            crossterm::event::KeyCode::PageDown => {
                (atoms::keycode(), atoms::page_down()).encode(env)
            }
            crossterm::event::KeyCode::Tab => (atoms::keycode(), atoms::tab()).encode(env),
            crossterm::event::KeyCode::BackTab => (atoms::keycode(), atoms::back_tab()).encode(env),
            crossterm::event::KeyCode::Delete => (atoms::keycode(), atoms::delete()).encode(env),
            crossterm::event::KeyCode::Insert => (atoms::keycode(), atoms::insert()).encode(env),
            crossterm::event::KeyCode::F(f) => (atoms::keycode(), (atoms::f(), f)).encode(env),
            crossterm::event::KeyCode::Char(c) => {
                (atoms::keycode(), (atoms::char(), c.to_string())).encode(env)
            }
            crossterm::event::KeyCode::Null => (atoms::keycode(), atoms::null()).encode(env),
            crossterm::event::KeyCode::Esc => (atoms::keycode(), atoms::esc()).encode(env),
            crossterm::event::KeyCode::CapsLock => {
                (atoms::keycode(), atoms::caps_lock()).encode(env)
            }
            crossterm::event::KeyCode::ScrollLock => {
                (atoms::keycode(), atoms::scroll_lock()).encode(env)
            }
            crossterm::event::KeyCode::NumLock => (atoms::keycode(), atoms::num_lock()).encode(env),
            crossterm::event::KeyCode::PrintScreen => {
                (atoms::keycode(), atoms::print_screen()).encode(env)
            }
            crossterm::event::KeyCode::Pause => (atoms::keycode(), atoms::pause()).encode(env),
            crossterm::event::KeyCode::Menu => (atoms::keycode(), atoms::menu()).encode(env),
            crossterm::event::KeyCode::KeypadBegin => {
                (atoms::keycode(), atoms::begin()).encode(env)
            }
            crossterm::event::KeyCode::Media(_media_key_code) => todo!(),
            crossterm::event::KeyCode::Modifier(_modifier_key_code) => todo!(),
        }
    }
}

struct KeyModifiers(crossterm::event::KeyModifiers);

impl From<crossterm::event::KeyModifiers> for KeyModifiers {
    fn from(value: crossterm::event::KeyModifiers) -> Self {
        Self(value)
    }
}

impl rustler::Encoder for KeyModifiers {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        match self.0 {
            // const CONTROL = 0b0000_0010;
            crossterm::event::KeyModifiers::CONTROL => atoms::control().to_term(env),
            // const SHIFT = 0b0000_0001;
            crossterm::event::KeyModifiers::SHIFT => atoms::shift().to_term(env),
            // const ALT = 0b0000_0100;
            crossterm::event::KeyModifiers::ALT => atoms::alt().to_term(env),
            // const SUPER = 0b0000_1000;
            crossterm::event::KeyModifiers::SUPER => {
                // super is already a rust keyword so we have to do it this way
                rustler::Atom::from_str(env, "super").unwrap().to_term(env)
            }
            // const HYPER = 0b0001_0000;
            crossterm::event::KeyModifiers::HYPER => atoms::hyper().to_term(env),
            // const META = 0b0010_0000;
            crossterm::event::KeyModifiers::META => atoms::meta().to_term(env),
            // const NONE = 0b0000_0000;
            crossterm::event::KeyModifiers::NONE => atoms::none().to_term(env),
            _ => todo!(),
        }
    }
}

struct KeyEventKind(crossterm::event::KeyEventKind);

impl From<crossterm::event::KeyEventKind> for KeyEventKind {
    fn from(value: crossterm::event::KeyEventKind) -> Self {
        Self(value)
    }
}

impl rustler::Encoder for KeyEventKind {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        match self.0 {
            crossterm::event::KeyEventKind::Press => atoms::press().to_term(env),
            crossterm::event::KeyEventKind::Repeat => atoms::repeat().to_term(env),
            crossterm::event::KeyEventKind::Release => atoms::release().to_term(env),
        }
    }
}

struct KeyEventState(crossterm::event::KeyEventState);

impl From<crossterm::event::KeyEventState> for KeyEventState {
    fn from(value: crossterm::event::KeyEventState) -> Self {
        Self(value)
    }
}

impl rustler::Encoder for KeyEventState {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        atoms::todo().to_term(env)
    }
}

#[rustler::nif(schedule = "DirtyIo")]
fn terminal_read_event() -> NifResult<(rustler::Atom, Event)> {
    let event = crossterm::event::read().map_err(|e| nif_error!(e))?;
    Ok((atoms::ok(), event.into()))
}

struct Constraint(ratatui::layout::Constraint);

impl<'a> rustler::Decoder<'a> for Constraint {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        let constraint: (rustler::Atom, u16) = term.decode()?;

        if constraint.0 == atoms::percentage() {
            Ok(Constraint(ratatui::layout::Constraint::Percentage(
                constraint.1,
            )))
        } else {
            Err(nif_error!("something bad"))
        }
    }
}

pub(crate) struct ChunksResource(pub Vec<Rect>);

impl Resource for ChunksResource {
    const IMPLEMENTS_DESTRUCTOR: bool = false;
    const IMPLEMENTS_DOWN: bool = false;
}

#[rustler::nif(schedule = "DirtyIo")]
fn chunks_new(
    terminal: ResourceArc<TerminalResource>,
    constraints: Vec<Constraint>,
) -> ResourceArc<ChunksResource> {
    let area = {
        let mut t = terminal.terminal.lock().unwrap();
        t.get_frame().area()
    };

    let rc_chunks = Layout::default()
        .constraints(constraints.iter().map(|c| c.0))
        .direction(Direction::Vertical)
        .split(area);

    // stupid stuff to move the chunks from the Rc to an Arc
    let chunks = {
        let mut out = Vec::with_capacity(rc_chunks.len());

        for chunk in rc_chunks.iter() {
            out.push(*chunk)
        }

        out
    };

    ResourceArc::new(ChunksResource(chunks))
}

// #[rustler::nif(schedule = "DirtyIo")]
// fn draw(terminal: ResourceArc<TerminalResource>, s: &str) -> NifResult<rustler::Atom> {
//     let constraints = [
//         ratatui::layout::Constraint::Percentage(60),
//         ratatui::layout::Constraint::Percentage(20),
//         ratatui::layout::Constraint::Percentage(10),
//     ];

//     let block = Block::default().borders(Borders::ALL).title(Span::styled(
//         "Info",
//         Style::default()
//             .fg(Color::Cyan)
//             .add_modifier(Modifier::BOLD),
//     ));

//     let mut terminal = terminal.terminal.lock().unwrap();

//     terminal
//         .draw(|frame| {
//             let chunks = Layout::default()
//                 .constraints(constraints)
//                 .direction(Direction::Vertical)
//                 .split(frame.area());

//             let paragraph = Paragraph::new(Text::from(s))
//                 .block(block)
//                 .wrap(Wrap { trim: false });

//             // terminal, widget, chunks, index
//             frame.render_widget(paragraph, chunks[0]);
//         })
//         .map_err(|e| nif_error!(e))?;

//     Ok(atoms::ok())
// }

#[rustler::nif(schedule = "DirtyIo", name = "terminal_predraw")]
fn predraw(terminal: ResourceArc<TerminalResource>) -> NifResult<()> {
    let mut terminal = terminal.terminal.lock().unwrap();
    terminal.autoresize().map_err(|e| nif_error!(e))
}

#[rustler::nif(schedule = "DirtyIo", name = "terminal_postdraw")]
fn postdraw(terminal: ResourceArc<TerminalResource>) -> NifResult<()> {
    let mut terminal = terminal.terminal.lock().unwrap();

    // let frame = terminal.get_frame();

    // We can't change the cursor position right away because we have to flush the frame to
    // stdout first. But we also can't keep the frame around, since it holds a &mut to
    // Buffer. Thus, we're taking the important data out of the Frame and dropping it.
    let cursor_position = terminal
        .backend_mut()
        .get_cursor_position()
        .map_err(|e| nif_error!(e));

    // Draw to stdout
    terminal.flush().map_err(|e| nif_error!(e))?;

    match cursor_position {
        Err(_) => terminal.hide_cursor().map_err(|e| nif_error!(e))?,
        Ok(position) => {
            terminal.show_cursor().map_err(|e| nif_error!(e))?;
            terminal
                .set_cursor_position(position)
                .map_err(|e| nif_error!(e))?;
        }
    }

    terminal.swap_buffers();

    // Flush
    terminal.backend_mut().flush().map_err(|e| nif_error!(e))?;

    // let completed_frame = ratatui::CompletedFrame {
    //     buffer: &self.buffers[1 - self.current],
    //     area: self.last_known_area,
    //     count: self.frame_count,
    // };

    // // increment frame count before returning from draw
    // self.frame_count = self.frame_count.wrapping_add(1);

    // Ok(completed_frame)
    Ok(())
}
// #[rustler::nif(schedule = "DirtyIo")]
// fn clear(terminal: ResourceArc<TerminalResource>) {
//     let mut terminal = terminal.terminal.lock().unwrap();
//     terminal.clear().unwrap();
// }

// #[rustler::nif(schedule = "DirtyIo")]
// fn resize(terminal: ResourceArc<TerminalResource>, width: u16, height: u16) -> NifResult<()> {
//     let mut terminal = terminal.terminal.lock().unwrap();
//     terminal
//         .resize(Rect {
//             x: 0,
//             y: 0,
//             width,
//             height,
//         })
//         .map_err(|e| nif_error!(e))
// }

// def autoresize(_terminal), do: :erlang.nif_error(:nif_not_loaded)
#[rustler::nif(schedule = "DirtyIo", name = "terminal_autoresize")]
fn autoresize(terminal: ResourceArc<TerminalResource>) -> NifResult<()> {
    let mut terminal = terminal.terminal.lock().unwrap();
    terminal.autoresize().map_err(|e| nif_error!(e))
}

// def get_cursor_position(_frame), do: :erlang.nif_error(:nif_not_loaded)
#[rustler::nif(schedule = "DirtyIo", name = "terminal_get_cursor_position")]
fn get_cursor_position(terminal: ResourceArc<TerminalResource>) -> NifResult<(u16, u16)> {
    let mut terminal = terminal.terminal.lock().unwrap();
    let position = terminal
        .backend_mut()
        .get_cursor_position()
        .map_err(|e| nif_error!(e))?;

    Ok((position.x, position.y))
}

// def flush(_terminal), do: :erlang.nif_error(:nif_not_loaded)
#[rustler::nif(schedule = "DirtyIo", name = "terminal_flush")]
fn flush(terminal: ResourceArc<TerminalResource>) -> NifResult<()> {
    let mut terminal = terminal.terminal.lock().unwrap();
    terminal.flush().map_err(|e| nif_error!(e))
}

// def hide_cursor(_terminal), do: :erlang.nif_error(:nif_not_loaded)
#[rustler::nif(schedule = "DirtyIo", name = "terminal_hide_cursor")]
fn hide_cursor(terminal: ResourceArc<TerminalResource>) -> NifResult<()> {
    let mut terminal = terminal.terminal.lock().unwrap();
    terminal.hide_cursor().map_err(|e| nif_error!(e))
}

// def show_cursor(_terminal), do: :erlang.nif_error(:nif_not_loaded)
#[rustler::nif(schedule = "DirtyIo", name = "terminal_show_cursor")]
fn show_cursor(terminal: ResourceArc<TerminalResource>) -> NifResult<()> {
    let mut terminal = terminal.terminal.lock().unwrap();
    terminal.show_cursor().map_err(|e| nif_error!(e))
}

// def set_cursor_position(_terminal, _position), do: :erlang.nif_error(:nif_not_loaded)
#[rustler::nif(schedule = "DirtyIo", name = "terminal_set_cursor_position")]
fn set_cursor_position(
    terminal: ResourceArc<TerminalResource>,
    position: (u16, u16),
) -> NifResult<()> {
    let mut terminal = terminal.terminal.lock().unwrap();
    terminal
        .set_cursor_position(position)
        .map_err(|e| nif_error!(e))
}

// def swap_buffers(_terminal), do: :erlang.nif_error(:nif_not_loaded)
#[rustler::nif(schedule = "DirtyIo", name = "terminal_swap_buffers")]
fn swap_buffers(terminal: ResourceArc<TerminalResource>) {
    let mut terminal = terminal.terminal.lock().unwrap();
    terminal.swap_buffers();
}

// def flush_backend(_terminal), do: :erlang.nif_error(:nif_not_loaded)
#[rustler::nif(schedule = "DirtyIo")]
fn terminal_flush_backend(terminal: ResourceArc<TerminalResource>) -> NifResult<()> {
    let mut terminal = terminal.terminal.lock().unwrap();
    // let mut backend = terminal.backend_mut();
    terminal.backend_mut().flush().map_err(|e| nif_error!(e))
}
// def increment_frame_count(_terminal), do: :erlang.nif_error(:nif_not_loaded)
// #[rustler::nif(schedule = "DirtyIo")]
// fn increment_frame_count(terminal: ResourceArc<TerminalResource>) -> NifResult<()> {
//     let mut terminal = terminal.terminal.lock().unwrap();
//     // let mut backend = terminal.backend_mut();
//     terminal.frame_count
// }
