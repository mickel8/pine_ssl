defmodule PineSSL do
  use Rustler, otp_app: :pine_ssl, crate: "pinessl"

  def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
end

defmodule MyStruct do
  defstruct a: 0
end
