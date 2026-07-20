-spec is_prefix_of_word(Sentence :: unicode:unicode_binary(), SearchWord :: unicode:unicode_binary()) -> integer().
is_prefix_of_word(Sentence, SearchWord) ->
    find_index_prefix(Sentence, SearchWord, 1).

find_index_prefix(<<>>, _, _) -> -1;
find_index_prefix(Sentence, SearchWord, Index) ->
    [H | T] = string:split(Sentence, <<" ">>),
    case is_binary_prefix(H, SearchWord) of
        true -> Index;
        false -> find_index_prefix(list_to_binary(T), SearchWord, Index + 1)
    end.

is_binary_prefix(_, <<>>) -> true;
is_binary_prefix(<<X, XT/binary>>, <<X, YT/binary>>) ->
    is_binary_prefix(XT, YT);
is_binary_prefix(_, _) -> false.
