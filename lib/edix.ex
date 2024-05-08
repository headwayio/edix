defmodule Edix do
  @moduledoc """
  Documentation for `Edix`.
  """

  @doc """
  Parse EDI file by pathname.

  ## Examples

      iex> Edix.parse_edi_file("sample.edi")
      {:ok, %EdixDocument{
                envelope: [
                  %Edix.EdixInterchangeControl{
                    authorization_information: "",
                    authorization_qualifier: "00",
                    security_qualifier: "00"
                  }
                ],
                element_delimiter: "*",
                segment_delimiter: "~",
                sub_element_delimiter: ">"
              }}

  """
  def parse_edi_file(path) do
    Edix.Parser.parse_edi_file(path)
  end
end
