-module(solution).
-export([main/0, parse_snafu/1, part1/1, print_snafu/1]).

parse_snafu([], Acc) -> Acc;
parse_snafu([D|Rest], Acc) -> 
  Digit = case D of
    $- -> -1;
    $= -> -2;
    D  -> D - $0
  end,
  parse_snafu(Rest, Acc * 5 + Digit).

-spec parse_snafu(string()) -> integer().
parse_snafu(Snafu) -> parse_snafu(Snafu, 0).

to_snafu_digit(-2) -> $=;
to_snafu_digit(-1) -> $-;
to_snafu_digit(D) -> D + $0.

rem_to_dig(4) -> -1;
rem_to_dig(3) -> -2;
rem_to_dig(D) -> D.

print_snafu(0, Acc) -> Acc;
print_snafu(Num, Acc) ->
  Rem = Num rem 5,
  % assume we don't print negative numbers
  Dig = rem_to_dig(Rem), %3 - Rem,
  Digit = to_snafu_digit(Dig),
  print_snafu((Num - Dig) div 5, [Digit|Acc]).

print_snafu(Num) -> print_snafu(Num, []).

part1(Filename) ->
  {ok, BinContents} = file:read_file(Filename),
  Contents = binary_to_list(BinContents),
  Lines = string:split(Contents, "\n", all),
  Nums = lists:map(fun parse_snafu/1, Lines),
  Sum = lists:sum(Nums),
  io:format("Sum: ~b~n", [Sum]),
  print_snafu(Sum).

main() -> part1("input.txt").
  
tests() -> 
  lists:filter(
    fun(I) -> I =:= solution:parse_snafu(solution:print_snafu(I)) end, 
    lists:seq(1, 10000)
  ).