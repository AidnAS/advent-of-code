defmodule Day3 do
  defp find_part_number(line, y_coord) do
    Regex.scan(~r/[0-9]+/, line, return: :index)
    |> Enum.flat_map(& &1)
    |> Enum.map(fn {start, length} ->
      %{
        :value => String.to_integer(String.slice(line, start, length)),
        :x_start => start,
        :x_end => start + length,
        :y_coord => y_coord
      }
    end)
  end

  defp find_parts(line, y_coord) do
    Regex.scan(~r/[^0-9.\n]/, line, return: :index)
    |> Enum.flat_map(& &1)
    |> Enum.map(fn {start, length} ->
      %{
        :part_symbol => String.slice(line, start, length),
        :x_coord => start,
        :y_coord => y_coord
      }
    end)
  end

  defp is_adjacent?(part_symbol, part_number)
       when abs(part_symbol.y_coord - part_number.y_coord) == 1 do
    part_symbol.x_coord >= part_number.x_start - 1 && part_symbol.x_coord <= part_number.x_end
  end

  defp is_adjacent?(part_symbol, part_number) when part_symbol.y_coord == part_number.y_coord do
    part_symbol.x_coord == part_number.x_start - 1 || part_symbol.x_coord == part_number.x_end
  end

  defp is_adjacent?(_part_symbol, _part_number) do
    false
  end

  defp is_adjacent_to_any_symbol?(part_number, part_symbols) do
    Enum.any?(part_symbols, &is_adjacent?(&1, part_number))
  end

  def solve(file) do
    schematic = File.stream!(file) |> Enum.to_list()

    part_symbols =
      schematic
      |> Enum.with_index(&find_parts(&1, &2))
      |> Enum.filter(&(length(&1) > 0))
      |> Enum.flat_map(& &1)

    schematic
    |> Enum.with_index(&find_part_number(&1, &2))
    |> Enum.filter(&(length(&1) > 0))
    |> Enum.flat_map(& &1)
    |> Enum.filter(&is_adjacent_to_any_symbol?(&1, part_symbols))
    |> Enum.map(& &1.value)
    |> Enum.sum()
  end
end

IO.puts(Day3.solve("input.txt"))
