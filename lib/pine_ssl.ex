defmodule PineSSL do
  use Rustler, otp_app: :pine_ssl, crate: "pinessl"

  def ssl_ctx_new(), do: :erlang.nif_error(:nif_not_loaded)

  def ssl_new(_ssl_ctx_ref), do: :erlang.nif_error(:nif_not_loaded)
end
