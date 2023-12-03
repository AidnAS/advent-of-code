# Which games would be possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?
# One game pr line, subsets separated by ;
# Max nr of cubes pr color across all subsets that are equal to or less than the input provided.

# Parse each line and extract max green, max red and max blue shown. -> { game_id: 9, red: 1, green: 2, blue: 3} (Is this a dict perhaps? )
# Split subsets by ; Do I need game index? Yes, because solution is sum of indexes.
#
defmodule Day2 do
  def max_reducer(cube_count, state) do
    [count_str, color_str] = String.split(cube_count, ~r/\s+/, trim: true)
    color = String.to_existing_atom(color_str)
    count = String.to_integer(count_str)

    %{
      :green => max(state.green, if(color == :green, do: count, else: 0)),
      :red => max(state.red, if(color == :red, do: count, else: 0)),
      :blue => max(state.blue, if(color == :blue, do: count, else: 0))
    }
  end

  def to_game(parts) do
    [head | tail] = parts
    index = String.to_integer(String.trim(head, "Game "))

    state = %{:green => 0, :red => 0, :blue => 0}

    max_cubes =
      tail
      |> Enum.map(&String.trim/1)
      |> Enum.flat_map(&String.split(&1, ","))
      |> Enum.reduce(state, &max_reducer/2)

    %{:game_index => index, :max_cubes => max_cubes}
  end

  def is_possible(available_cubes, game_config) do
    game_config.max_cubes.green <= available_cubes.green and
      game_config.max_cubes.red <= available_cubes.red and
      game_config.max_cubes.blue <= available_cubes.blue
  end

  def solve(file, available_cubes) do
    possible_games =
      File.stream!(file)
      |> Stream.map(&String.trim/1)
      |> Stream.map(&String.split(&1, ~r{:|;}))
      |> Stream.map(&to_game(&1))
      |> Stream.filter(&is_possible(available_cubes, &1))
      |> Stream.map(& &1.game_index)

    Enum.to_list(possible_games)
    |> Enum.sum()
  end
end

IO.puts(Day2.solve("input.txt", %{:red => 12, :green => 13, :blue => 14}))
