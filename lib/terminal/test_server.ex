defmodule Terminal.TestServer do
  use GenServer
  require Logger

  defstruct [:terminal, :tick_rate, :last_tick, :app_state, :chunks, :exit_buf]

  def start_link(args) do
    GenServer.start_link(__MODULE__, args, name: __MODULE__)
  end

  @impl GenServer
  def init(_args) do
    state = %__MODULE__{tick_rate: 50}
    {:ok, state, {:continue, :start_terminal}}
  end

  @impl GenServer
  def handle_continue(:start_terminal, state) do
    Logger.debug("up")
    {:ok, terminal} = Terminal.new(state.tick_rate, :passive)
    last_tick = DateTime.utc_now()

    Process.send(self(), :poll, [])

    constraints = [
      {:percentage, 60},
      {:percentage, 20},
      {:percentage, 10}
    ]

    chunks = Terminal.chunks(terminal, constraints)

    state = %__MODULE__{
      state
      | terminal: terminal,
        last_tick: last_tick,
        app_state: "",
        chunks: chunks,
        exit_buf: []
    }

    Terminal.draw(state.terminal, fn _terminal ->
      Terminal.render_paragraph(state.terminal, state.app_state, state.chunks, 0)
      Terminal.render_paragraph(state.terminal, state.app_state, state.chunks, 1)
      Terminal.render_paragraph(state.terminal, state.app_state, state.chunks, 2)
    end)

    {
      :noreply,
      state
    }
  end

  @impl GenServer
  def handle_info(:poll, state) do
    e = elapsed(state.last_tick)
    tick_time = state.tick_rate - e

    state =
      with {_, {:ok, true}} <- {:event_available?, Terminal.event_available?(tick_time)},
           {_, {:ok, {:event, event}}} <- {:read_event, Terminal.read_event()} do
        case event do
          %{code: {:keycode, {:char, "c"}}, modifiers: :control} ->
            # state = %__MODULE__{app_state: "got Ctrl-C, quitting..."}

            Terminal.draw(state.terminal, fn terminal ->
              Terminal.render_paragraph(terminal, state.app_state, state.chunks, 0)
            end)

            System.stop(0)

            Logger.debug("#{inspect(state.exit_buf)}")

            # Process.sleep(:infinity)

            state

          %{code: {:keycode, :backspace}} ->
            state = %__MODULE__{
              state
              | app_state: String.slice(state.app_state, 0..-2//1)
            }

            Terminal.draw(state.terminal, fn _terminal ->
              Terminal.render_paragraph(state.terminal, state.app_state, state.chunks, 0)
              Terminal.render_paragraph(state.terminal, state.app_state, state.chunks, 1)
              Terminal.render_paragraph(state.terminal, state.app_state, state.chunks, 2)
            end)

            state

          %{code: {:keycode, :enter}} ->
            state = %__MODULE__{
              state
              | app_state: "#{state.app_state}\n"
            }

            Terminal.draw(state.terminal, fn _terminal ->
              Terminal.render_paragraph(state.terminal, state.app_state, state.chunks, 0)
              Terminal.render_paragraph(state.terminal, state.app_state, state.chunks, 1)
              Terminal.render_paragraph(state.terminal, state.app_state, state.chunks, 2)
            end)

            state

          %{code: {:keycode, {:char, c}}} ->
            state = %__MODULE__{
              state
              | app_state: "#{state.app_state}#{c}"
            }

            Terminal.draw(state.terminal, fn _terminal ->
              Terminal.render_paragraph(state.terminal, state.app_state, state.chunks, 0)
              Terminal.render_paragraph(state.terminal, state.app_state, state.chunks, 1)
              Terminal.render_paragraph(state.terminal, state.app_state, state.chunks, 2)
            end)

            state

          {:resize, _width, _height} ->
            # Terminal.autoresize(state.terminal)

            # state = %__MODULE__{state | exit_buf: ["autoresized to #{x} #{y}" | state.exit_buf]}

            Terminal.autoresize(state.terminal)

            Terminal.draw(state.terminal, fn _terminal ->
              Terminal.render_paragraph(state.terminal, state.app_state, state.chunks, 0)
              Terminal.render_paragraph(state.terminal, state.app_state, state.chunks, 1)
              Terminal.render_paragraph(state.terminal, state.app_state, state.chunks, 2)
            end)

            state

          # catchall for keycodes not yet handled
          %{code: {:keycode, _}} ->
            Terminal.draw(state.terminal, fn _terminal ->
              Terminal.render_paragraph(state.terminal, state.app_state, state.chunks, 0)
              Terminal.render_paragraph(state.terminal, state.app_state, state.chunks, 1)
              Terminal.render_paragraph(state.terminal, state.app_state, state.chunks, 2)
            end)

            state
        end
      else
        {:event_available?, {:ok, false}} ->
          state

        e ->
          state = %__MODULE__{state | exit_buf: [e | state.exit_buf]}

          Terminal.draw(state.terminal, fn _terminal ->
            Terminal.render_paragraph(state.terminal, state.app_state, state.chunks, 0)
            Terminal.render_paragraph(state.terminal, state.app_state, state.chunks, 1)
            Terminal.render_paragraph(state.terminal, state.app_state, state.chunks, 2)
          end)

          state
      end

    state =
      if elapsed(state.last_tick) >= state.tick_rate do
        %__MODULE__{state | last_tick: DateTime.utc_now()}
      else
        state
      end

    Process.send_after(self(), :poll, tick_time, [])

    {:noreply, state}
  end

  @impl GenServer
  def handle_info({:event, event}, state) do
    state =
      case event do
        :tick ->
          Terminal.draw(state.terminal, fn _terminal ->
            Terminal.render_paragraph(state.terminal, state.app_state, state.chunks, 0)
          end)

          state

        %{code: {:keycode, {:char, "c"}}, modifiers: "Control"} ->
          # state = %__MODULE__{app_state: "got Ctrl-C, quitting..."}

          # Terminal.draw(state.terminal, fn terminal ->
          #   Terminal.render_paragraph(terminal, state.app_state, state.chunks, 0)
          # end)

          System.stop(0)

          Process.sleep(:infinity)

          state

        %{code: {:keycode, {:char, c}}} ->
          state = %__MODULE__{
            state
            | app_state: "you pressed #{c}"
          }

          # Terminal.draw(state.terminal, state.app_state)
          # block =
          #   Terminal.new_block() |> Terminal.block_borders() |> Terminal.block_title("SOME TITLE")

          Terminal.draw(state.terminal, fn _terminal ->
            Terminal.render_paragraph(state.terminal, state.app_state, state.chunks, 0)
          end)

          state

        _ ->
          state
      end

    {:noreply, state}
  end

  def handle_info(m, state) do
    raise m
  end

  def elapsed(datetime) do
    DateTime.diff(DateTime.utc_now(), datetime)
  end
end
