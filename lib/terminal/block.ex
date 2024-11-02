defmodule Terminal.Block do
  defdelegate new, to: Terminal.Native, as: :block_new
  # def block_new(), do: :erlang.nif_error(:nif_not_loaded)

  defdelegate borders(block), to: Terminal.Native, as: :block_borders
  # @doc false
  # def block_borders(_block), do: :erlang.nif_error(:nif_not_loaded)

  # @doc false
  # def block_title(_block, _title), do: :erlang.nif_error(:nif_not_loaded)
  defdelegate title(block, title), to: Terminal.Native, as: :block_title
end
