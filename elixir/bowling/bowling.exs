defmodule Bowling do
  alias Bowling.Frame

  @max_pin_count 10
  @max_frame_count 10

  @type t :: %Bowling{pin_count: pos_integer, frames: nonempty_list(Frame.t())}
  defstruct pin_count: @max_pin_count, frames: []

  defmodule Frame do
    @type t :: %Frame{
            rolls: list(pos_integer),
            type: :open | :spare | :strike | :undetermined,
            finished: boolean
          }
    defstruct rolls: [], type: :undetermined, finished: false

    @max_pin_count 10

    @spec create(pos_integer) :: %Frame{}
    def create(roll) do
      case roll do
        @max_pin_count -> %Frame{rolls: [roll], type: :strike}
        roll -> %Frame{rolls: [roll]}
      end
    end

    @spec update(%Frame{}, pos_integer) :: %Frame{} | {:error, String.t()}
    def update(%Frame{type: :spare, rolls: rolls, finished: false} = frame, roll) do
      %{frame | rolls: [roll | rolls], finished: true}
    end

    def update(%Frame{type: :strike, rolls: rolls, finished: false} = frame, roll)
        when length(rolls) == 2 do
      %{frame | rolls: [roll | rolls], finished: true}
    end

    def update(%Frame{type: :strike, rolls: rolls, finished: false} = frame, roll)
        when length(rolls) == 1 do
      %{frame | rolls: [roll | rolls]}
    end

    def update(%Frame{type: :undetermined, rolls: [], finished: false} = frame, @max_pin_count) do
      %{frame | rolls: [@max_pin_count], type: :strike}
    end

    def update(%Frame{type: :undetermined, rolls: [], finished: false} = frame, roll) do
      %{frame | rolls: [roll]}
    end

    def update(
          %Frame{type: :undetermined, rolls: [first_roll | []] = rolls, finished: false} = frame,
          roll
        )
        when first_roll + roll == @max_pin_count do
      %{frame | rolls: [roll | rolls], type: :spare}
    end

    def update(
          %Frame{type: :undetermined, rolls: [first_roll | []] = rolls, finished: false} = frame,
          roll
        )
        when first_roll + roll != @max_pin_count do
      %{frame | rolls: [roll | rolls], type: :open, finished: true}
    end
  end

  @doc """
    Creates a new game of bowling that can be used to store the results of
    the game
  """

  @spec start() :: Bowling.t()
  def start do
    %Bowling{}
  end

  @doc """
    Records the number of pins knocked down on a single roll. Returns `any`
    unless there is something wrong with the given number of pins, in which
    case it returns a helpful message.
  """

  @spec roll(%Bowling{}, pos_integer) :: %Bowling{} | {:error, String.t()}
  def roll(_, roll) when is_integer(roll) and roll < 0,
    do: {:error, "Negative roll is invalid"}

  def roll(%Bowling{pin_count: pin_count}, roll)
      when is_integer(roll) and pin_count - roll < 0,
      do: {:error, "Pin count exceeds pins on the lane"}

  def roll(%Bowling{frames: [%Frame{finished: true} | _] = frames}, _)
      when length(frames) == @max_frame_count,
      do: {:error, "Cannot roll after game is over"}

  def roll(%Bowling{} = game, roll) when is_integer(roll) do
    case {game, roll} do
      # First roll of the game
      {%Bowling{frames: []}, roll} ->
        game
        |> update_pin_count(roll)
        |> new_frame(roll)

      # two unfinished frames
      {%Bowling{
         frames: [%Frame{finished: false}, %Frame{finished: false} | _]
       }, roll} ->
        game
        |> update_pin_count(roll)
        |> update_last_two_frames(roll)
        |> new_frame_if_necessary(game, roll)

      # one unfinished frame
      {%Bowling{frames: [%Frame{finished: false} | _]}, roll} ->
        game
        |> update_pin_count(roll)
        |> update_last_frame(roll)
        |> new_frame_if_necessary(game, roll)

      # zero unfinished frames
      {%Bowling{frames: [%Frame{finished: true} | _]}, roll} ->
        game
        |> update_pin_count(roll)
        |> new_frame(roll)
    end
  end

  @doc """
    Returns the score of a given game of bowling if the game is complete.
    If the game isn't complete, it returns a helpful message.
  """

  @spec score(%Bowling{}) :: pos_integer | {:error, String.t()}
  def score(%Bowling{frames: [%Frame{finished: false} | _]}) do
    {:error, "Score cannot be taken until the end of the game"}
  end

  def score(%Bowling{frames: frames}) when length(frames) < 10 do
    {:error, "Score cannot be taken until the end of the game"}
  end

  def score(%Bowling{frames: frames}) when length(frames) == 10 do
    Enum.reduce(frames, 0, fn frame, acc -> Enum.sum(frame.rolls) + acc end)
  end

  @spec new_frame(%Bowling{}, pos_integer) :: %Bowling{}
  defp new_frame(%Bowling{frames: frames} = game, roll) do
    %{game | frames: [Frame.create(roll) | frames]}
  end

  defp new_frame_if_necessary(
         %Bowling{} = game,
         %Bowling{frames: [%Frame{type: type, finished: false} | _] = frames},
         roll
       ) do
    case {length(frames), type} do
      {@max_frame_count, _} -> game
      {_, :undetermined} -> game
      _ -> game |> new_frame(roll)
    end
  end

  @spec update_last_two_frames(%Bowling{}, pos_integer) :: %Bowling{}
  defp update_last_two_frames(
         %Bowling{
           frames: [%Frame{finished: false} = head_1, %Frame{finished: false} = head_2 | tail]
         } = game,
         roll
       ) do
    %{game | frames: [Frame.update(head_1, roll), Frame.update(head_2, roll) | tail]}
  end

  defp update_last_frame(%Bowling{frames: [%Frame{finished: false} = head | tail]} = game, roll) do
    %{game | frames: [Frame.update(head, roll) | tail]}
  end

  @spec update_pin_count(%Bowling{}, pos_integer) :: %Bowling{}
  defp update_pin_count(
         %Bowling{pin_count: pin_count, frames: frames} = game,
         roll
       ) do
    case frames do
      [%Frame{type: :undetermined} | _] -> %{game | pin_count: @max_pin_count}
      _ when pin_count - roll == 0 -> %{game | pin_count: @max_pin_count}
      _ when pin_count - roll > 0 -> %{game | pin_count: pin_count - roll}
      [] -> %{game | pin_count: pin_count - roll}
    end
  end
end
