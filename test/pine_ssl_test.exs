defmodule PineSslTest do
  use ExUnit.Case

  test "ssl_ctx_new" do
    assert is_reference(PineSSL.ssl_ctx_new())
  end

  test "ssl_new" do
    ssl_ctx_ref = PineSSL.ssl_ctx_new()
    assert is_reference(ssl_ctx_ref)
    assert is_reference(PineSSL.ssl_new(ssl_ctx_ref))
  end
end
