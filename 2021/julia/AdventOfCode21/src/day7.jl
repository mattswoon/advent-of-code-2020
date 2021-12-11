module Day7

TESTINPUT = "16,1,2,0,4,2,7,1,2,14"

single_cost(pos::Int64, target::Int64) = abs(pos - target)
cost(positions::Vector{Int64}, target::Int64) = single_cost.(positions, target) |> sum
start(ps::Vector{Int64}) = (maximum(ps) - minimum(ps)) / 2 |> floor |> Int64

function gradient_descent(func, start::Int64)
    mid = func(start)
    pre = func(start - 1)
    post = func(start + 1)
    if (mid <= pre) && (mid <= post)
        return (start, mid)
    elseif pre < mid < post
        return gradient_descent(func, start - 1)
    elseif pre > mid > post
        return gradient_descent(func, start + 1)
    end
    throw("wat?")
end

run_part1(s) = joinpath(s, "day7.txt") |>
    f -> read(f, String) |>
    s -> parse.(Int64, split(s, ",")) |>
    ps -> (y -> cost(ps, y), start(ps)) |>
    x -> gradient_descent(x...) |>
    t -> "Optimal position is $(t[1]) with a cost of $(t[2])"

single_cost_2(pos::Int64, target::Int64) = (abs(pos - target) / 2) * (1 + abs(pos - target)) |> Int64
cost_2(positions::Vector{Int64}, target::Int64) = single_cost_2.(positions, target) |> sum

run_part2(s) = joinpath(s, "day7.txt") |>
    f -> read(f, String) |>
    s -> parse.(Int64, split(s, ",")) |>
    ps -> (y -> cost_2(ps, y), start(ps)) |>
    x -> gradient_descent(x...) |>
    t -> "Optimal position is $(t[1]) with a cost of $(t[2])"

end
