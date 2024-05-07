defmodule Edix.Parser do
  use RustlerPrecompiled,
    otp_app: :edix,
    crate: "edix_parser",
    base_url: "https://github.com/headwayio/edix/releases/download/#{Edix.MixProject.version}",
    version: Edix.MixProject.version

  # When your NIF is loaded, it will override this function.
  def main(), do: :erlang.nif_error(:nif_not_loaded)

  # When your NIF is loaded, it will override this function.
  def parse_edi_file(_path), do: :erlang.nif_error(:nif_not_loaded)
end

