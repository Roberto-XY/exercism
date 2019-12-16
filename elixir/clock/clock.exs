defmodule Clock do
  defstruct hour: 0, minute: 0

  @minute_base 60
  @hour_base 24

  @doc """
  Returns a clock that can be represented as a string:

      iex> Clock.new(8, 9) |> to_string
      "08:09"
  """
  @spec new(integer, integer) :: Clock
  def new(hour, minute) when is_integer(hour) and is_integer(minute) do
    %Clock{
      hour: calculate_hour(hour * @minute_base + minute),
      minute: calculate_minute(minute)
    }
  end
  
  @spec calculate_hour(integer) :: pos_integer
  defp calculate_hour(minute) do
    hour = rem(div(minute, @minute_base), @hour_base)

    cond do
      rem(minute, @minute_base) < 0 -> fix_negative_time(hour - 1, @hour_base)
      hour < 0 -> fix_negative_time(hour, @hour_base)
      true -> hour
    end
  end

  @spec calculate_minute(integer) :: pos_integer
  defp calculate_minute(minute) do
    rem(minute, @minute_base) |> fix_negative_time(@minute_base)
  end

  @spec fix_negative_time(integer, pos_integer) :: pos_integer
  defp fix_negative_time(x, base) do
    rem(base + x, base)
  end

  @doc """
  Adds two clock times:

      iex> Clock.new(10, 0) |> Clock.add(3) |> to_string
      "10:03"
  """
  @spec add(Clock, integer) :: Clock
  def add(%Clock{hour: hour, minute: minute}, add_minute) when is_integer(add_minute) do
    Clock.new(hour, minute + add_minute)
  end
end

defimpl String.Chars, for: Clock do
  @spec to_string(Clock) :: String.t()
  def to_string(%Clock{} = clock) do
    "#{prettify(clock.hour)}:#{prettify(clock.minute)}"
  end

  @spec prettify(integer) :: String.t()
  defp prettify(int) do
    int
    |> Integer.to_string()
    |> String.pad_leading(2, "0")
  end
end
