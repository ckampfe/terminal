defmodule Terminal.CallbackServer do
  use GenServer
  require Logger

  def start_link(args) do
    GenServer.start_link(__MODULE__, args, name: __MODULE__)
  end

  @impl GenServer
  def init(args) do
    {:ok, args}
  end

  @impl GenServer
  def handle_info({:execute_callback, {fun, args, future} = body}, state) do
    Logger.debug(inspect(body))

    result = fun.(args)

    Logger.debug(result: result)

    Terminal.complete_future(future, result)

    {:noreply, state}
  end
end
