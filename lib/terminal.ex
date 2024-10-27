defmodule Terminal do
  use Rustler,
    otp_app: :terminal,
    crate: :terminal

  def new(_tick_rate, _mode), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  If this returns {:ok, true}, there is an event available, and you may call
  read_event, and it will not block.

  If it returns {:ok, false}, there is no event available, so calling read_event
  *will block* the caller.
  """
  def event_available?(_milliseconds), do: :erlang.nif_error(:nif_not_loaded)
  def read_event(), do: :erlang.nif_error(:nif_not_loaded)
  # def clear(_terminal), do: :erlang.nif_error(:nif_not_loaded)
  # def draw(_terminal, _s), do: :erlang.nif_error(:nif_not_loaded)
  def new_block(), do: :erlang.nif_error(:nif_not_loaded)
  def block_borders(_block), do: :erlang.nif_error(:nif_not_loaded)
  def block_title(_block, _title), do: :erlang.nif_error(:nif_not_loaded)
  def chunks(_terminal, _constraints), do: :erlang.nif_error(:nif_not_loaded)

  def new_paragraph(_block, _text), do: :erlang.nif_error(:nif_not_loaded)

  def render_paragraph(_terminal, _text, _chunks, _chunk_index),
    do: :erlang.nif_error(:nif_not_loaded)

  # def resize(_terminal, _width, _height), do: :erlang.nif_error(:nif_not_loaded)

  # terminal: ResourceArc<TerminalResource>,
  #   text: String,
  #   chunks: ResourceArc<ChunksResource>,
  #   index: usize,

  def draw(terminal, f) do
    :ok = try_draw(terminal, f)
    nil
  end

  def try_draw(terminal, f) do
    predraw(terminal)

    f.(terminal)

    postdraw(terminal)

    :ok
  end

  def predraw(_terminal), do: :erlang.nif_error(:nif_not_loaded)
  def postdraw(_terminal), do: :erlang.nif_error(:nif_not_loaded)

  # def try_draw(terminal, f) do
  #   # statuses = []
  #   # statuses = [1 | statuses]
  #   autoresize(terminal)

  #   # statuses = [2 | statuses]
  #   f.(terminal)

  #   # statuses = [3 | statuses]
  #   cursor_position = get_cursor_position(terminal)

  #   # statuses = [4 | statuses]
  #   flush(terminal)

  #   # statuses = [5 | statuses]

  #   # statuses =
  #   case cursor_position do
  #     nil ->
  #       # statuses = [6 | statuses]
  #       hide_cursor(terminal)

  #     # statuses

  #     position ->
  #       # statuses = [7 | statuses]
  #       show_cursor(terminal)
  #       # statuses = [8 | statuses]
  #       set_cursor_position(terminal, position)
  #       # statuses
  #   end

  #   # statuses = [9 | statuses]
  #   swap_buffers(terminal)

  #   # statuses = [10 | statuses]
  #   flush_backend(terminal)

  #   # IO.inspect(statuses)

  #   # can't increment frame count it's a private api
  #   # increment_frame_count(terminal)
  #   :ok
  # end

  # pub fn try_draw<F, E>(&mut self, render_callback: F) -> io::Result<CompletedFrame>
  #   where
  #       F: FnOnce(&mut Frame) -> Result<(), E>,
  #       E: Into<io::Error>,
  #   {
  #       // Autoresize - otherwise we get glitches if shrinking or potential desync between widgets
  #       // and the terminal (if growing), which may OOB.
  #       self.autoresize()?;

  #       let mut frame = self.get_frame();

  #       render_callback(&mut frame).map_err(Into::into)?;

  #       // We can't change the cursor position right away because we have to flush the frame to
  #       // stdout first. But we also can't keep the frame around, since it holds a &mut to
  #       // Buffer. Thus, we're taking the important data out of the Frame and dropping it.
  #       let cursor_position = frame.cursor_position;

  #       // Draw to stdout
  #       self.flush()?;

  #       match cursor_position {
  #           None => self.hide_cursor()?,
  #           Some(position) => {
  #               self.show_cursor()?;
  #               self.set_cursor_position(position)?;
  #           }
  #       }

  #       self.swap_buffers();

  #       // Flush
  #       self.backend.flush()?;

  #       let completed_frame = CompletedFrame {
  #           buffer: &self.buffers[1 - self.current],
  #           area: self.last_known_area,
  #           count: self.frame_count,
  #       };

  #       // increment frame count before returning from draw
  #       self.frame_count = self.frame_count.wrapping_add(1);

  #       Ok(completed_frame)
  #   }

  def autoresize(_terminal), do: :erlang.nif_error(:nif_not_loaded)
  # def get_frame(_terminal), do: :erlang.nif_error(:nif_not_loaded)
  def get_cursor_position(_terminal), do: :erlang.nif_error(:nif_not_loaded)
  def flush(_terminal), do: :erlang.nif_error(:nif_not_loaded)
  def hide_cursor(_terminal), do: :erlang.nif_error(:nif_not_loaded)
  def show_cursor(_terminal), do: :erlang.nif_error(:nif_not_loaded)
  def set_cursor_position(_terminal, _position), do: :erlang.nif_error(:nif_not_loaded)
  def swap_buffers(_terminal), do: :erlang.nif_error(:nif_not_loaded)
  def flush_backend(_terminal), do: :erlang.nif_error(:nif_not_loaded)
  # def increment_frame_count(_terminal), do: :erlang.nif_error(:nif_not_loaded)
end
