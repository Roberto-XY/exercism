defmodule BinarySearch do
  @doc """
    Searches for a key in the tuple using the binary search algorithm.
    It returns :not_found if the key is not in the tuple.
    Otherwise returns {:ok, index}.

    ## Examples

      iex> BinarySearch.search({}, 2)
      :not_found

      iex> BinarySearch.search({1, 3, 5}, 2)
      :not_found

      iex> BinarySearch.search({1, 3, 5}, 5)
      {:ok, 2}

  """

  @spec search(tuple, integer) :: {:ok, integer} | :not_found
  def search(numbers, key) when is_tuple(numbers) and is_integer(key) do
    numbers = Tuple.to_list(numbers) |> Enum.sort()
    search_properly(numbers, key)
  end

  @spec search(list, integer) :: {:ok, integer} | :not_found
  defp search_properly(_, _, head_index \\ 0)
  defp search_properly([], _, _), do: :not_found
  defp search_properly([head | []], key, head_index) when head == key, do: {:ok, head_index}
  defp search_properly([head | []], key, _) when head != key, do: :not_found

  defp search_properly(list, key, head_index) do
    middle = (length(list) / 2) |> round

    {left, [right_head | right_tail]} = Enum.split(list, middle)
    right_head_index = head_index + length(left)

    cond do
      key == right_head -> {:ok, right_head_index}
      key < right_head -> search_properly(left, key, head_index)
      key > right_head -> search_properly(right_tail, key, right_head_index + 1)
    end
  end
end
