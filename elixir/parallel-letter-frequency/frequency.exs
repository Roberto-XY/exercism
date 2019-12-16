defmodule Frequency do
  alias LetterCount

  @doc """
  Count letter frequency in parallel.

  Returns a map of characters to frequencies.

  The number of worker processes to use can be set with 'workers'.
  """
  @spec frequency([String.t()], pos_integer) :: %{char => pos_integer}
  def frequency([], workers) when is_integer(workers) and workers > 0, do: %{}

  def frequency(strings, workers) when is_list(strings) and is_integer(workers) and workers > 0 do
    strings
    |> Task.async_stream(&LetterCount.count_letters(&1))
    |> Enum.reduce(%{}, fn {:ok, map}, acc -> LetterCount.merge_results(map, acc) end)
  end
end

defmodule LetterCountSupervisor do
  use Supervisor

  def start_link(init_arg) do
    Supervisor.start_link(__MODULE__, init_arg, name: __MODULE__)
  end

  @impl true
  def init(_init_arg) do
    children = [
      {LetterCountTask, [:hello]}
    ]

    Supervisor.init(children, strategy: :one_for_one)
  end
end

defmodule LetterCountTask do
  use Task, restart: :transient
end

defmodule LetterCount do
  @alphabet ~r/\p{L}/u

  @spec count_letters([String.t()] | String.t()) :: %{char => pos_integer}
  def count_letters(strings) when is_list(strings) do
    strings
    |> Enum.reduce(%{}, fn string, acc -> merge_results(acc, count_letters(string)) end)
  end

  def count_letters(string) when is_binary(string) do
    string
    |> String.graphemes()
    |> Enum.reduce(%{}, fn char, acc ->
      Map.update(acc, String.downcase(char), 1, &(&1 + 1))
    end)
    |> filter_alphabet()
  end

  @spec merge_results(%{char => pos_integer}, %{char => pos_integer}) :: %{char => pos_integer}
  def merge_results(result1, result2) when is_map(result1) and is_map(result2) do
    Map.merge(result1, result2, fn _k, v1, v2 -> merge_counts(v1, v2) end)
  end

  @spec merge_counts(pos_integer, pos_integer) :: pos_integer
  defp merge_counts(count1, count2) when is_integer(count1) and is_integer(count2) do
    count1 + count2
  end

  @spec filter_alphabet(%{char => pos_integer}) :: %{char => pos_integer}
  defp filter_alphabet(result) when is_map(result) do
    key_strings =
      result
      |> Map.keys()
      |> Enum.filter(&Regex.match?(@alphabet, &1))

    Map.take(result, key_strings)
  end
end
