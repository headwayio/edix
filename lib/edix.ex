defmodule Edix do
  @moduledoc """
  Documentation for `Edix`.
  """

  @doc """
  Parse EDI file by pathname.

  ## Examples

      iex> {:ok, edix_document} = Edix.parse_edi_file("sample.edi")
      ...> edix_document.segment_delimiter
      "~"

  """
  def parse_edi_file(path) do
    Edix.Parser.parse_edi_file(path)
  end
end
