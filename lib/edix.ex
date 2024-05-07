defmodule Edix do
  @moduledoc """
  Documentation for `Edix`.
  """

  @doc """
  Hello world.

  ## Examples

      iex> Edix.hello()
      :world

  """
  def hello do
    :world
  end

  @doc """
  Parse EDI file by pathname.

  ## Examples

      iex> Edix.parse_edi_file("sample.edi")
      {:ok, %EdixDocument{name: "915792US00"}}

  """
  def parse_edi_file(path) do
    Edix.Parser.parse_edi_file(path)
  end
end
