defmodule Solution do
  @spec count_pairs(nums :: [integer], k :: integer) :: integer
  def count_pairs(nums, k) do
    count_pairs(nums, k, 0, 0)
  end

  defp count_pairs([], _k, _i, acc), do: acc

  defp count_pairs([head | tail], k, i, acc) do
    new_acc =
        Enum.with_index(tail, i + 1)
        |> Enum.reduce(acc, fn {val, j}, a ->
            if head == val and rem(i * j, k) == 0 do
                a + 1
            else
                a
            end
        end)

    count_pairs(tail, k, i + 1, new_acc)
  end
end