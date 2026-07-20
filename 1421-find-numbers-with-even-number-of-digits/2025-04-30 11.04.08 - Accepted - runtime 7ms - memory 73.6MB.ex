require Integer
require String

defmodule Solution do
  @spec find_numbers(nums :: [integer]) :: integer
  def find_numbers(nums) do
    nums
    |> Enum.map(&Integer.to_string/1)
    |> Enum.map(&String.length/1)
    |> Enum.filter(&Integer.is_even/1)
    |> Enum.count()
  end
end
