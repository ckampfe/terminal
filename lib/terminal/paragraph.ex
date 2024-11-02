defmodule Terminal.Paragraph do
  defdelegate new(block, text), to: Terminal.Native, as: :paragraph_new

  defdelegate render(terminal, text, chunks, chunks_index),
    to: Terminal.Native,
    as: :paragraph_render
end
