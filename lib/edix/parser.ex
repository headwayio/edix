defmodule Edix.Parser do
  @moduledoc """
  Placeholder definition for Rust NIFs
  """

  use RustlerPrecompiled,
    otp_app: :edix,
    crate: "edix_parser",
    base_url: "https://github.com/headwayio/edix/releases/download/#{Edix.MixProject.version()}",
    version: Edix.MixProject.version()

  # When your NIF is loaded, each function below will be overridden

  def parse(_file_content), do: :erlang.nif_error(:nif_not_loaded)
  def parse_file(_path), do: :erlang.nif_error(:nif_not_loaded)
end
