defmodule Edix do
  @moduledoc """
  Documentation for `Edix`.
  """

  @doc """
  Parse EDI file by pathname.

  ## Examples

      iex> {:ok, edix_document} = Edix.parse_file("sample.edi")
      ...> edix_document.segment_delimiter
      "~"

  """
  def parse_file(path) do
    Edix.Parser.parse_file(path)
  end

  @doc """
  Parse EDI string.

  ## Examples

      iex> {:ok, edix_document} = Edix.parse("my*edi*content")
      ...> edix_document.segment_delimiter
      "~"

  """
  def parse(content) do
    Edix.Parser.parse(content)
  end
end
