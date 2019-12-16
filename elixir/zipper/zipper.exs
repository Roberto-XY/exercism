defmodule BinTree do
  import Inspect.Algebra

  @moduledoc """
  A node in a binary tree.

  `value` is the value of a node.
  `left` is the left subtree (nil if no subtree).
  `right` is the right subtree (nil if no subtree).
  """
  @type t :: %BinTree{value: any, left: BinTree.t() | nil, right: BinTree.t() | nil}
  defstruct [:value, :left, :right]

  # A custom inspect instance purely for the tests, this makes error messages
  # much more readable.
  #
  # BinTree[value: 3, left: BinTree[value: 5, right: BinTree[value: 6]]] becomes (3:(5::(6::)):)
  def inspect(%BinTree{value: value, left: left, right: right}, opts) do
    concat([
      "(",
      to_doc(value, opts),
      ":",
      if(left, do: to_doc(left, opts), else: ""),
      ":",
      if(right, do: to_doc(right, opts), else: ""),
      ")"
    ])
  end
end

defmodule Zipper do
  @type trail :: {:left, any, BinTree.t(), trail} | {:right, any, BinTree.t(), trail} | :top

  @type t :: %Zipper{
          value: any,
          left: BinTree.t() | nil,
          right: BinTree.t() | nil,
          trail: Zipper.trail()
        }
  defstruct [:value, :left, :right, :trail]

  @doc """
  Get a zipper focused on the root node.
  """
  @spec from_tree(BinTree.t()) :: Zipper.t()
  def from_tree(%BinTree{value: v, left: l, right: r}) do
    %Zipper{value: v, left: l, right: r, trail: :top}
  end

  @doc """
  Get the complete tree from a zipper.
  """
  @spec to_tree(Zipper.t()) :: BinTree.t()
  def to_tree(%Zipper{value: v, left: l, right: r, trail: :top}),
    do: %BinTree{value: v, left: l, right: r}

  def to_tree(%Zipper{} = zipper), do: to_tree(up(zipper))

  @doc """
  Get the value of the focus node.
  """
  @spec value(Zipper.t()) :: any
  def value(%Zipper{value: value}) do
    value
  end

  @doc """
  Get the left child of the focus node, if any.
  """
  @spec left(Zipper.t()) :: Zipper.t() | nil
  def left(%Zipper{left: nil}), do: nil

  def left(%Zipper{
        value: v,
        left: %BinTree{value: left_v, left: left_l, right: left_r},
        right: r,
        trail: t
      }) do
    %Zipper{value: left_v, left: left_l, right: left_r, trail: {:right, v, r, t}}
  end

  @doc """
  Get the right child of the focus node, if any.
  """
  @spec right(Zipper.t()) :: Zipper.t() | nil
  def right(%Zipper{right: nil}), do: nil

  def right(%Zipper{
        value: v,
        left: l,
        right: %BinTree{value: right_v, left: right_l, right: right_r},
        trail: t
      }) do
    %Zipper{value: right_v, left: right_l, right: right_r, trail: {:left, v, l, t}}
  end

  @doc """
  Get the parent of the focus node, if any.
  """
  @spec up(Zipper.t()) :: Zipper.t() | nil
  def up(%Zipper{trail: :top}), do: nil

  def up(%Zipper{
        value: v,
        left: l,
        right: r,
        trail: {:left, left_trail_v, left_trail_bin_tree, trail}
      }) do
    %Zipper{
      value: left_trail_v,
      left: left_trail_bin_tree,
      right: %BinTree{value: v, left: l, right: r},
      trail: trail
    }
  end

  def up(%Zipper{
        value: v,
        left: l,
        right: r,
        trail: {:right, right_trail_v, right_trail_bin_tree, trail}
      }) do
    %Zipper{
      value: right_trail_v,
      left: %BinTree{value: v, left: l, right: r},
      right: right_trail_bin_tree,
      trail: trail
    }
  end

  @doc """
  Set the value of the focus node.
  """
  @spec set_value(Zipper.t(), any) :: Zipper.t()
  def set_value(%Zipper{} = zipper, value) do
    %{zipper | value: value}
  end

  @doc """
  Replace the left child tree of the focus node.
  """
  @spec set_left(Zipper.t(), BinTree.t() | nil) :: Zipper.t()
  def set_left(%Zipper{} = zipper, nil), do: %{zipper | left: nil}
  def set_left(%Zipper{} = zipper, %BinTree{} = left), do: %{zipper | left: left}

  @doc """
  Replace the right child tree of the focus node.
  """
  @spec set_right(Zipper.t(), BinTree.t() | nil) :: Zipper.t()
  def set_right(%Zipper{} = zipper, nil), do: %{zipper | right: nil}
  def set_right(%Zipper{} = zipper, %BinTree{} = right), do: %{zipper | right: right}
end
