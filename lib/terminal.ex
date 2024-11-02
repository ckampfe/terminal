defmodule Terminal do
  alias Terminal.Native

  defdelegate new(tick_rate, mode), to: Terminal.Native, as: :terminal_new

  defdelegate autoresize(terminal), to: Terminal.Native, as: :terminal_autoresize

  @spec read_event() :: {:ok, term()} | {:error, binary()}
  defdelegate read_event(), to: Terminal.Native, as: :terminal_read_event

  @spec event_available?(pos_integer()) :: {:ok, boolean()} | {:error, binary()}
  defdelegate event_available?(milliseconds), to: Terminal.Native, as: :terminal_event_available?

  def draw(terminal, f) do
    :ok = try_draw(terminal, f)
    nil
  end

  def try_draw(terminal, f) do
    Native.terminal_predraw(terminal)

    f.(terminal)

    Native.terminal_postdraw(terminal)

    :ok
  end
end
