defmodule PineSSL.MixProject do
  use Mix.Project

  def project do
    [
      app: :pine_ssl,
      version: "0.1.0",
      elixir: "~> 1.11",
      compilers: [:rustler] ++ Mix.compilers(),
      rustler_crates: [pinessl: []],
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.22.0-rc.0"}
    ]
  end
end
