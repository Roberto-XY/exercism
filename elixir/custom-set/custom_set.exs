defmodule CustomSet do
  @opaque t(value) :: %CustomSet{map: %{optional(value) => []}}
  @type t :: t(any)

  defstruct map: %{}

  @spec new(Enum.t()) :: t
  def new(enumerable) do
    map =
      enumerable
      |> Enum.to_list()
      |> new_from_list()

    %CustomSet{map: map}
  end

  @spec new_from_list(list(), Enum.t()) :: %{optional(any) => []}
  defp new_from_list(list, acc \\ [])

  defp new_from_list([], acc) do
    :maps.from_list(acc)
  end

  defp new_from_list([element | tail], acc) do
    new_from_list(tail, [{element, []} | acc])
  end

  @spec empty?(t) :: boolean
  def empty?(%CustomSet{map: map}) do
    map_size(map) == 0
  end

  @spec contains?(t, any) :: boolean
  def contains?(%CustomSet{map: map}, element) do
    case Map.get(map, element) do
      [] -> true
      nil -> false
    end
  end

  @spec subset?(t, t) :: boolean
  def subset?(%CustomSet{map: map1}, %CustomSet{map: map2}) do
    Map.keys(map1)
    |> Enum.all?(fn element -> match?(%{^element => _}, map2) end)
  end

  @spec disjoint?(t, t) :: boolean
  def disjoint?(%CustomSet{map: map1}, %CustomSet{map: map2}) do
    Map.keys(map1)
    |> Enum.all?(fn element -> not match?(%{^element => _}, map2) end)
  end

  @spec equal?(t, t) :: boolean
  def equal?(%CustomSet{map: map1}, %CustomSet{map: map2}) do
    Map.equal?(map1, map2)
  end

  @spec add(t, any) :: t
  def add(%CustomSet{map: map} = set, element) do
    %{set | map: Map.put(map, element, [])}
  end

  @spec intersection(t, t) :: t
  def intersection(%CustomSet{map: map1}, %CustomSet{map: map2}) do
    %CustomSet{map: Map.take(map2, Map.keys(map1))}
  end

  @spec difference(t, t) :: t
  def difference(%CustomSet{map: map1}, %CustomSet{map: map2}) do
    %CustomSet{map: Map.drop(map1, Map.keys(map2))}
  end

  @spec union(t, t) :: t
  def union(%CustomSet{map: map1}, %CustomSet{map: map2}) do
    %CustomSet{map: Map.merge(map1, map2)}
  end
end
