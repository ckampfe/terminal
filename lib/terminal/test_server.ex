defmodule Terminal.TestServer do
  use GenServer
  require Logger

  defstruct [:terminal, :tick_rate, :last_tick, :app_state]

  def start_link(args) do
    GenServer.start_link(__MODULE__, args, name: __MODULE__)
  end

  @impl GenServer
  def init(_args) do
    state = %__MODULE__{tick_rate: 100}
    {:ok, state, {:continue, :start_terminal}}
  end

  @impl GenServer
  def handle_continue(:start_terminal, state) do
    Logger.debug("up")
    {:ok, terminal} = Terminal.new(state.tick_rate, :active)
    last_tick = DateTime.utc_now()

    # Process.send(self(), :poll, [])

    {
      :noreply,
      %{state | terminal: terminal, last_tick: last_tick, app_state: ""}
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
          %{"code" => {:keycode, {:char, "c"}}, "modifiers" => "Control"} ->
            Terminal.draw(state.terminal, "Got control C, quitting!")
            Process.sleep(1000)
            System.stop(0)

          %{"code" => {:keycode, {:char, c}}} ->
            state = %{
              state
              | app_state:
                  state.app_state <> "\nyou pressed #{c}, tick_time: #{tick_time}, elapsed: #{e}"
            }

            Terminal.draw(state.terminal, state.app_state)

            state

          _ ->
            state
        end
      else
        _e ->
          Terminal.draw(state.terminal, state.app_state)
          state
      end

    state =
      if elapsed(state.last_tick) >= state.tick_rate do
        %{state | last_tick: DateTime.utc_now()}
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
          Terminal.draw(state.terminal, state.app_state)
          state

        %{code: {:keycode, {:char, "c"}}, modifiers: "Control"} ->
          Terminal.draw(state.terminal, "Got control C, quitting!")
          Process.sleep(1000)
          System.stop(0)

        %{code: {:keycode, {:char, c}}} ->
          state = %{
            state
            | app_state: state.app_state <> "\nyou pressed #{c}"
          }

          Terminal.draw(state.terminal, state.app_state)

          state

        _ ->
          state
      end

    {:noreply, state}
  end

  def elapsed(datetime) do
    DateTime.diff(DateTime.utc_now(), datetime)
  end
end
