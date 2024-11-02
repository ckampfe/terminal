defmodule Terminal.Native do
  use Rustler,
    otp_app: :terminal,
    crate: :terminal

  # BEGIN TERMINAL #

  @doc false
  def terminal_new(_tick_rate, _mode), do: :erlang.nif_error(:nif_not_loaded)

  @doc false
  def terminal_event_available?(_milliseconds), do: :erlang.nif_error(:nif_not_loaded)

  @doc false
  def terminal_read_event(), do: :erlang.nif_error(:nif_not_loaded)

  @doc false
  def terminal_predraw(_terminal), do: :erlang.nif_error(:nif_not_loaded)

  @doc false
  def terminal_postdraw(_terminal), do: :erlang.nif_error(:nif_not_loaded)

  @doc false
  def terminal_autoresize(_terminal), do: :erlang.nif_error(:nif_not_loaded)

  @doc false
  def terminal_get_cursor_position(_terminal), do: :erlang.nif_error(:nif_not_loaded)

  @doc false
  def terminal_flush(_terminal), do: :erlang.nif_error(:nif_not_loaded)

  @doc false
  def terminal_hide_cursor(_terminal), do: :erlang.nif_error(:nif_not_loaded)

  @doc false
  def terminal_show_cursor(_terminal), do: :erlang.nif_error(:nif_not_loaded)

  @doc false
  def terminal_set_cursor_position(_terminal, _position), do: :erlang.nif_error(:nif_not_loaded)

  @doc false
  def terminal_swap_buffers(_terminal), do: :erlang.nif_error(:nif_not_loaded)

  @doc false
  def terminal_flush_backend(_terminal), do: :erlang.nif_error(:nif_not_loaded)

  # END TERMINAL #

  ####################################################

  # BEGIN PARAGRAPH #

  @doc false
  def paragraph_new(_block, _text), do: :erlang.nif_error(:nif_not_loaded)

  @doc false
  def paragraph_render(_terminal, _text, _chunks, _chunk_index),
    do: :erlang.nif_error(:nif_not_loaded)

  # END PARAGRAPH #

  ####################################################

  # BEGIN BLOCK #

  @doc false
  def block_new(), do: :erlang.nif_error(:nif_not_loaded)

  @doc false
  def block_borders(_block), do: :erlang.nif_error(:nif_not_loaded)

  @doc false
  def block_title(_block, _title), do: :erlang.nif_error(:nif_not_loaded)

  # END BLOCK #

  # BEGIN CHUNKS #

  def chunks_new(_terminal, _constraints), do: :erlang.nif_error(:nif_not_loaded)

  # END CHUNKS

  ####################################################

  # def lock_terminal_with_callback(_terminal, _f, _args), do: :erlang.nif_error(:nif_not_loaded)
  # def complete_future(_future, _result), do: :erlang.nif_error(:nif_not_loaded)
end
