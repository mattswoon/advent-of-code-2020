module Day8

run_part2(s) = "not done"

SEGMENT_COUNTS = Dict(2 => [1],
                      3 => [7],
                      4 => [4],
                      5 => [2, 3, 5],
                      6 => [0, 6, 9],
                      7 => [8])

possible_digits_by_segment_count(s) = SEGMENT_COUNTS[length(s)]

output_values(s) = split(s, "|")[2] |> 
    strip |>
    y -> split(y, " ")

run_part1(data_dir) = joinpath(data_dir, "day8.txt") |>
    readlines |>
    ls -> output_values.(ls) |>
    Base.Iterators.flatten |>
    collect |>
    ovs -> possible_digits_by_segment_count.(ovs) |>
    ds -> length.(ds) |>
    ds -> filter(y -> y == 1, ds) |>
    sum

DISPLAY = Dict(Set("abcefg") => 0,
               Set("cf") => 1,
               Set("acdeg") => 2,
               Set("acdfg") => 3,
               Set("bcdf") => 4,
               Set("abdfg") => 5,
               Set("abdefg") => 6,
               Set("acf") => 7,
               Set("abcdefg") => 8,
               Set("abcdfg") => 9)

struct Permuation
    a
    b
    c
    d
    e
    f
    g
end

end
