defmodule Solution do
  @spec prefix_count(words :: [String.t], pref :: String.t) :: integer
  def prefix_count(words, pref) do
    Enum.count(words, fn word -> String.starts_with?(word, pref) end)
  end
end