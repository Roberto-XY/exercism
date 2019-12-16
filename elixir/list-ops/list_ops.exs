defmodule ListOps do
  # Please don't use any external modules (especially List or Enum) in your
  # implementation. The point of this exercise is to create these basic
  # functions yourself. You may use basic Kernel functions (like `Kernel.+/2`
  # for adding numbers), but please do not use Kernel functions for Lists like
  # `++`, `--`, `hd`, `tl`, `in`, and `length`.

  @spec count(list) :: non_neg_integer
  def count([]), do: 0
  def count([_ | []]), do: 1
  def count([_ | tail]), do: 1 + count(tail)

  @spec reverse(list) :: list
  def reverse(l) when is_list(l), do: reduce(l, [], &[&1 | &2])

  @spec map(list, (any -> any)) :: list
  def map(l, f) when is_list(l) and is_function(f, 1) do
    l |> reverse() |> reduce([], &[f.(&1) | &2])
  end

  @spec filter(list, (any -> as_boolean(term))) :: list
  def filter(l, f) when is_list(l) and is_function(f, 1) do
    l |> reverse() |> reduce([], &if(f.(&1), do: [&1 | &2], else: &2))
  end

  @type acc :: any
  @spec reduce(list, acc, (any, acc -> acc)) :: acc
  def reduce([], acc, f) when is_function(f, 2), do: acc
  def reduce([head | tail], acc, f) when is_function(f, 2), do: reduce(tail, f.(head, acc), f)

  @spec append(list, list) :: list
  def append(l1, l2) when is_list(l1) and is_list(l2) do
    l1 |> reverse() |> reduce(l2, &[&1 | &2])
  end

  @spec concat([[any]]) :: [any]
  def concat(ll) when is_list(ll), do: ll |> reverse() |> reduce([], &append/2)
end
