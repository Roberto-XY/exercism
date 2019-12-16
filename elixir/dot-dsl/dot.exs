defmodule Graph do
  defstruct attrs: [], nodes: [], edges: []
end

defmodule Dot do
  defmacro graph(do: nil) do
    Macro.escape(%Graph{})
  end

  defmacro graph(do: block) do
    build_graph(%Graph{}, block)
    |> sort_entries()
    |> Macro.escape()
  end

  @spec sort_entries(Graph.t()) :: Graph.t()
  defp sort_entries(%Graph{} = graph) do
    %Graph{
      attrs: Enum.sort(graph.attrs),
      nodes: Enum.sort(graph.nodes),
      edges: Enum.sort(graph.edges)
    }
  end

  # DSL list
  @spec build_graph(Graph.t(), {atom(), list(any()), list(any())}) :: Graph.t()
  defp build_graph(%Graph{} = graph, {:__block__, _, dsl_lines}) do
    Enum.reduce(dsl_lines, graph, fn dsl_line, g -> build_graph(g, dsl_line) end)
  end

  # attrs
  defp build_graph(%Graph{} = graph, {:graph, _, [attributes]}) do
    if Keyword.keyword?(attributes) do
      %{graph | attrs: graph.attrs ++ attributes}
    else
      raise ArgumentError
    end
  end

  # nodes
  defp build_graph(%Graph{} = graph, {name, _, [attributes]}) do
    if Keyword.keyword?(attributes) do
      %{graph | nodes: graph.nodes ++ [{name, attributes}]}
    else
      raise ArgumentError
    end
  end

  defp build_graph(%Graph{} = graph, {name, _, nil}) do
    %{graph | nodes: graph.nodes ++ [{name, []}]}
  end

  # edges
  defp build_graph(
         %Graph{} = graph,
         {:--, _, [{start_node, _, nil}, {end_node, _, [attributes]}]}
       ) do
    if Keyword.keyword?(attributes) do
      %{graph | edges: graph.edges ++ [{start_node, end_node, attributes}]}
    else
      raise ArgumentError
    end
  end

  defp build_graph(
         %Graph{} = graph,
         {:--, _, [{start_node, _, nil}, {end_node, _, nil}]}
       ) do
    %{graph | edges: graph.edges ++ [{start_node, end_node, []}]}
  end

  defp build_graph(%Graph{}, ast_triple) do
    raise ArgumentError
  end
end
