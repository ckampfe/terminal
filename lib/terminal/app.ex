defmodule Terminal.App do
  @type event :: term()
  @type app_state :: term()
  @type terminal :: term()

  @callback setup(terminal(), app_state()) :: app_state()
  @doc """
  Given an event and the previous state, return a new state.
  """
  @callback update(event(), app_state()) :: app_state()
  @callback draw(terminal(), app_state()) :: :ok | {:error, term()}
  @optional_callbacks setup: 2

  defstruct [:terminal, :tick_rate, :last_tick, :app_state, :chunks, :exit_buf, :timer_ref]

  defmacro __using__(options) do
    default_tick_rate = 50

    quote location: :keep,
          bind_quoted: [
            options: options,
            module: __CALLER__.module,
            default_tick_rate: default_tick_rate
          ] do
      @behaviour Terminal.App

      use GenServer

      def start_link(args) do
        GenServer.start_link(__MODULE__, args, name: unquote(options)[:name] || __MODULE__)
      end

      @impl GenServer
      def init(_args) do
        state = %Terminal.App{
          tick_rate: unquote(options)[:tick_rate] || unquote(default_tick_rate),
          app_state: struct(__MODULE__)
        }

        {:ok, state, {:continue, :start_terminal}}
      end

      @impl GenServer
      def handle_continue(:start_terminal, state) do
        {:ok, terminal} = Terminal.new(state.tick_rate, :passive)

        state = %Terminal.App{state | terminal: terminal}

        new_app_state = unquote(module).setup(state.terminal, state.app_state)

        timer_ref = Process.send_after(self(), :tick, state.tick_rate)

        last_tick = DateTime.utc_now()

        {
          :noreply,
          %Terminal.App{
            state
            | app_state: new_app_state,
              terminal: terminal,
              last_tick: last_tick,
              timer_ref: timer_ref
          }
        }
      end

      @impl GenServer
      def handle_info(:tick, %Terminal.App{timer_ref: timer_ref} = state) do
        Process.cancel_timer(timer_ref)

        e = Terminal.App.elapsed(state.last_tick)
        tick_time = Terminal.App.clamp(state.tick_rate - e, 1, state.tick_rate)

        # I have no idea why dialyzer doesn't like this `with`
        new_app_state =
          with {:event_available?, {:ok, true}} <-
                 {:event_available?, Terminal.event_available?(tick_time)},
               {:read_event, {:ok, event}} <- {:read_event, Terminal.read_event()} do
            unquote(module).update(event, state.app_state)
          else
            {:event_available?, {:ok, false}} ->
              unquote(module).update(:tick, state.app_state)

            {:read_event, {:error, error}} ->
              raise error

            {:event_available?, {:error, error}} ->
              raise error
          end

        case unquote(module).draw(state.terminal, new_app_state) do
          :ok -> nil
          {:error, error} -> raise error
        end

        new_state =
          if e >= state.tick_rate do
            %Terminal.App{state | last_tick: DateTime.utc_now()}
          else
            state
          end

        timer_ref = Process.send_after(self(), :tick, tick_time, [])

        {:noreply, %Terminal.App{state | app_state: new_app_state, timer_ref: timer_ref}}
      end

      @impl GenServer
      def handle_info({ref, result}, %Terminal.App{timer_ref: timer_ref} = state) do
        Process.demonitor(ref, [:flush])
        Process.cancel_timer(timer_ref)

        e = Terminal.App.elapsed(state.last_tick)

        tick_time = state.tick_rate - e

        new_app_state = unquote(module).update({:task, {ref, result}}, state.app_state)

        case unquote(module).draw(state.terminal, new_app_state) do
          :ok -> nil
          {:error, error} -> raise error
        end

        new_state =
          if e >= state.tick_rate do
            %Terminal.App{state | last_tick: DateTime.utc_now()}
          else
            state
          end

        timer_ref = Process.send_after(self(), :tick, tick_time, [])

        {:noreply, %Terminal.App{state | app_state: new_app_state, timer_ref: timer_ref}}
      end

      def setup(_terminal, app_state), do: app_state

      defoverridable setup: 2
    end
  end

  def elapsed(datetime) do
    DateTime.diff(DateTime.utc_now(), datetime)
  end

  def clamp(value, min, max) do
    cond do
      value < min -> min
      value > max -> max
      true -> value
    end
  end
end
