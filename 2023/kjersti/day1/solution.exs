defmodule Day1 do
  def extract_numbers(line) do
    pattern = ~r/(?=(1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine))/

    Regex.scan(pattern, line)
    |> Enum.map(fn match -> Enum.filter(match, &(&1 != "")) end)
    |> List.flatten()
  end

  def to_num(value) do
    case value do
      n when n in ["1", "one"] ->
        "1"

      n when n in ["2", "two"] ->
        "2"

      n when n in ["3", "three"] ->
        "3"

      n when n in ["4", "four"] ->
        "4"

      n when n in ["5", "five"] ->
        "5"

      n when n in ["6", "six"] ->
        "6"

      n when n in ["7", "seven"] ->
        "7"

      n when n in ["8", "eight"] ->
        "8"

      n when n in ["9", "nine"] ->
        "9"
    end
  end

  def calibration_value(nums) do
    first = List.first(nums) |> to_num()
    last = List.last(nums) |> to_num()
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
    |> Enum.map(&IO.inspect/1)
    |> Enum.sum()
  end
end

IO.puts(Day1.solve())
