defmodule SecretHandshake do
  @doc """
  Determine the actions of a secret handshake based on the binary
  representation of the given `code`.

  If the following bits are set, include the corresponding action in your list
  of commands, in order from lowest to highest.

  1 = wink
  10 = double blink
  100 = close your eyes
  1000 = jump

  10000 = Reverse the order of the operations in the secret handshake
  """
  @spec commands(code :: integer) :: list(String.t())
  def commands(code) do
    []
    |> wink(<<code::5>>)
    |> double_blink(<<code::5>>)
    |> close_your_eyes(<<code::5>>)
    |> jump(<<code::5>>)
    |> reverse(<<code::5>>)
  end

  defp wink(list, <<_::4, 1::1>>), do: list ++ ["wink"]
  defp wink(list, _code), do: list

  defp double_blink(list, <<_::3, 1::1, _::1>>), do: list ++ ["double blink"]
  defp double_blink(list, _code), do: list

  defp close_your_eyes(list, <<_::2, 1::1, _::2>>), do: list ++ ["close your eyes"]
  defp close_your_eyes(list, _code), do: list

  defp jump(list, <<_::1, 1::1, _::3>>), do: list ++ ["jump"]
  defp jump(list, _code), do: list

  defp reverse(list, <<1::1, _::4>>), do: Enum.reverse(list)
  defp reverse(list, _code), do: list
end
