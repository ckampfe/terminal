defmodule Terminal.Chunks do
  defdelegate new(terminal, constraints), to: Terminal.Native, as: :chunks_new
end
