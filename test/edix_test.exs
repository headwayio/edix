defmodule EdixTest do
  use ExUnit.Case

  alias Edix.EdixDocument

  doctest Edix

  test "parse edi file" do
    assert Edix.parse_edi_file("./sample.edi") ==
             {:ok,
              %EdixDocument{
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
  end
end
