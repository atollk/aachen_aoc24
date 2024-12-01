defmodule ElixirAocTest do
  use ExUnit.Case
  doctest ElixirAoc

  test "greets the world" do
    assert ElixirAoc.hello() == :world
  end
end
