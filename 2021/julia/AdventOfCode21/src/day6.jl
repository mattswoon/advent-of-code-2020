module Day6

TESTINPUT = """3,4,3,1,2"""

struct LanternFish
    timer::Int64
end

LanternFish(s::String) = LanternFish.(parse.(Int64, split(s, ",")))

age(l::LanternFish) = l.timer == 0 ? LanternFish(6) : LanternFish(l.timer - 1)
spawn(l::LanternFish) = l.timer == 0 ? LanternFish(8) : nothing
step_day(ls::Vector{LanternFish}) = vcat(age.(ls), filter(!isnothing, spawn.(ls)) |> x -> convert(Vector{LanternFish}, x))
simulate(ls::Vector{LanternFish}, days::Int64) = days == 0 ? ls : simulate(step_day(ls), days - 1)

run_part1(data_dir::String) = joinpath(data_dir, "day6.txt") |>
    f -> read(f, String) |>
    LanternFish |>
    ls -> simulate(ls, 80) |>
    length

mutable struct LanternFishEcosystem
    state::Dict{Int64, Int64}
end

function LanternFishEcosystem(ls::Vector{LanternFish})
    state = Dict(i => 0 for i in 0:8)
    for l in ls
        state[l.timer] += 1
    end
    LanternFishEcosystem(state)
end

function step_day(eco::LanternFishEcosystem)
    Dict(7 => eco.state[8],
         6 => eco.state[7] + eco.state[0],
         5 => eco.state[6],
         4 => eco.state[5],
         3 => eco.state[4],
         2 => eco.state[3],
         1 => eco.state[2],
         0 => eco.state[1],
         8 => eco.state[0]) |> LanternFishEcosystem
end
simulate(ls::LanternFishEcosystem, days::Int64) = days == 0 ? ls : simulate(step_day(ls), days - 1)

Base.length(eco::LanternFishEcosystem) = values(eco.state) |> sum


run_part2(data_dir::String) = joinpath(data_dir, "day6.txt") |>
    f -> read(f, String) |>
    LanternFish |>
    LanternFishEcosystem |>
    eco -> simulate(eco, 256) |>
    length

end # module
