module AdventOfCode21

include("./day1.jl")
include("./day2.jl")
include("./day3.jl")
include("./day4.jl")
include("./day5.jl")

function run(data_dir::String)
    run_bit("Day 1 (part 1)", Day1.run_part1, data_dir)
    run_bit("Day 1 (part 2)", Day1.run_part2, data_dir)
    run_bit("Day 2 (part 1)", Day2.run_part1, data_dir)
    run_bit("Day 2 (part 2)", Day2.run_part2, data_dir)
    run_bit("Day 3 (part 1)", Day3.run_part1, data_dir)
    run_bit("Day 3 (part 2)", Day3.run_part2, data_dir)
    run_bit("Day 4 (part 1)", Day4.run_part1, data_dir)
    run_bit("Day 4 (part 2)", Day4.run_part2, data_dir)
    run_bit("Day 5 (part 1)", Day5.run_part1, data_dir)
    run_bit("Day 5 (part 2)", Day5.run_part2, data_dir)
end

function run_bit(name::String, func::Function, data_dir::String)
    printstyled("$name:", bold=true, color=:green)
    println("\t\t$(func(data_dir))")
end

end # module
