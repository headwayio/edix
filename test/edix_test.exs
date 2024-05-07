defmodule EdixTest do
  use ExUnit.Case

  alias Edix.EdixDocument

  doctest Edix

  test "greets the world" do
    assert Edix.hello() == :world
  end

  test "parse edi file" do
    assert Edix.parse_edi_file("./sample.edi") == {:ok, %EdixDocument{name: "915792US00"}}
  end
end
