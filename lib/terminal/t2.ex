defmodule Terminal.T2 do
  require Logger

  alias Terminal.{Chunks, Paragraph}

  use Terminal.App

  defstruct [
    :chunks,
    paragraph_text: "",
    should_resize?: false,
    should_quit?: false,
    unknown_events: []
  ]

  @impl Terminal.App
  def setup(terminal, state) do
    constraints = [
      {:percentage, 40},
      {:percentage, 30},
      {:percentage, 30}
    ]

    chunks = Chunks.new(terminal, constraints)

    state = %__MODULE__{state | chunks: chunks}

    draw(terminal, state)

    state
  end

  @impl Terminal.App
  def update(_, %__MODULE__{should_quit?: true} = state) do
    System.stop(0)
    Process.sleep(:infinity)
    state
  end

  def update(%{code: {:keycode, {:char, "c"}}, modifiers: :control}, state) do
    %__MODULE__{state | paragraph_text: "Quitting!", should_quit?: true}
  end

  def update(%{code: {:keycode, :backspace}, modifiers: :none, kind: :press}, state) do
    %__MODULE__{state | paragraph_text: String.slice(state.paragraph_text, 0..-2//1)}
  end

  def update(%{code: {:keycode, :enter}, modifiers: :none, kind: :press}, state) do
    Task.Supervisor.async_nolink(T2.TaskSupervisor, fn ->
      Process.sleep(:timer.seconds(5))
      {:some_enter_message, "foo"}
    end)

    %__MODULE__{state | paragraph_text: state.paragraph_text <> "\n"}
  end

  def update(%{code: {:keycode, {:char, c}}}, state) do
    %__MODULE__{state | paragraph_text: state.paragraph_text <> c}
  end

  def update({:resize, _width, _height}, state) do
    %__MODULE__{state | should_resize?: true}
  end

  def update(:tick, state) do
    state
  end

  def update(event, state) do
    %__MODULE__{state | unknown_events: [event | state.unknown_events]}
  end

  @impl Terminal.App
  def draw(terminal, state) do
    state =
      if state.should_resize? do
        Terminal.autoresize(terminal)
        %__MODULE__{state | should_resize?: false}
      else
        state
      end

    Terminal.draw(terminal, fn t ->
      Paragraph.render(t, state.paragraph_text, state.chunks, 0)
      Paragraph.render(t, state.paragraph_text, state.chunks, 1)

      Paragraph.render(
        t,
        state.unknown_events
        |> Enum.map(fn e -> inspect(e) end)
        |> Enum.join("\n"),
        state.chunks,
        2
      )
    end)

    :ok
  end
end
