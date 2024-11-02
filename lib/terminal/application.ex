defmodule Terminal.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      # Starts a worker by calling: Terminal.Worker.start_link(arg)
      # {Terminal.Worker, arg}
      # {Terminal.CallbackServer, []}
      # {Terminal.TestServer, []}
      {Task.Supervisor, name: T2.TaskSupervisor},
      {Terminal.T2, []}
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: Terminal.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
