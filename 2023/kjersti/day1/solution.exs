defmodule Day1 do
  def extract_numbers(line) do
    pattern = ~r/(1|2|3|4|5|6|7|8|9|0)/

    Regex.scan(pattern, line, capture: :first)
    |> List.flatten()
  end

  def calibration_value(nums) do
    first = List.first(nums)
    last = List.last(nums)
    {value, ""} = Integer.parse(first <> last)
    value
  end

  def solve do
    calibration_parts =
      File.stream!("input.txt")
      |> Stream.map(&String.trim/1)
      |> Stream.map(&extract_numbers/1)
      |> Stream.map(&calibration_value/1)

    Enum.to_list(calibration_parts)
    |> Enum.sum()
  end
end

IO.puts(Day1.solve())
