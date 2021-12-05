module Day3

read(f::String) = map(s -> parse.(Bool, Vector{Char}(s)), readlines(f))

function gamma(arr::Vector{BitVector})
    N = length(arr) / 2 |> ceil |> Int64
    totals = sum(arr)
    totals .>= N
end

epsilon(gamma::BitVector) = length(gamma) |> ones |> BitVector |> x -> (x - gamma) |> BitVector

base10(bin::BitVector) = sum(d * 2^(i-1) for (i, d) in enumerate(reverse(bin)))

run_part1(data_dir::String) = joinpath(data_dir, "day3.txt") |>
    read |>
    gamma |>
    g -> (g, epsilon(g)) |>
    x -> mapreduce(base10, *, x)

abstract type MostCommonBit end

struct Zero <: MostCommonBit end
struct One <: MostCommonBit end
struct Equal <: MostCommonBit end

to_bit(b::MostCommonBit, default::Bool) = typeof(b) == Zero ? false : (typeof(b) == One ? true : default)

function most_common_value(arr::Vector{BitVector}, pos)
    num_ones = map(x -> x[pos], arr) |> sum
    num_zeros = length(arr) - num_ones
    num_zeros > num_ones ? Zero() : (num_zeros == num_ones ? Equal() : One())
end

function least_common_value(arr::Vector{BitVector}, pos)
    num_ones = map(x -> x[pos], arr) |> sum
    num_zeros = length(arr) - num_ones
    num_zeros < num_ones ? Zero() : (num_zeros == num_ones ? Equal() : One())
end

oxygen_filter(arr::Vector{BitVector}, pos, mcv::MostCommonBit) = filter(x -> to_bit(mcv, true) == x[pos], arr)
co2_filter(arr::Vector{BitVector}, pos, lcv::MostCommonBit) = filter(x -> to_bit(lcv, false) == x[pos], arr)

function oxygen(arr::Vector{BitVector}, pos=1)
    if length(arr) == 1
        return arr[1]
    else
        mcv = most_common_value(arr, pos)
        return oxygen_filter(arr, pos, mcv) |> y -> oxygen(y, pos + 1)
    end
end


function co2(arr::Vector{BitVector}, pos=1)
    if length(arr) == 1
        return arr[1]
    else
        lcv = least_common_value(arr, pos)
        return co2_filter(arr, pos, lcv) |> y -> co2(y, pos + 1)
    end
end

run_part2(data_dir::String) = joinpath(data_dir, "day3.txt") |>
    read |>
    arr -> (oxygen(arr), co2(arr)) |>
    x -> mapreduce(base10, *, x)
    
end # module
