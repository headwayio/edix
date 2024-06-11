defmodule Edix.MixProject do
  use Mix.Project

  @version "0.2.1"

  def project do
    [
      app: :edix,
      version: @version,
      elixir: "~> 1.15",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      package: package()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  def version, do: @version

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      # {:dep_from_hexpm, "~> 0.3.0"},
      # {:dep_from_git, git: "https://github.com/elixir-lang/my_dep.git", tag: "0.1.0"}
      {:rustler_precompiled, "~> 0.7.1", runtime: false},
      {:rustler, ">= 0.0.0", optional: true},
      {:credo, "~> 1.7", only: [:dev, :test], runtime: false}
    ]
  end

  defp package do
    [
      files: [
        "lib",
        "native/edix_parser/.cargo",
        "native/edix_parser/src",
        "native/edix_parser/Cargo*",
        "checksum-*.exs",
        "mix.exs"
      ]
    ]
  end
end
