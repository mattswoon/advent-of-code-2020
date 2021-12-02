module AdventOfCode21

include("./day1.jl")

function run(data_dir::String)
    run_bit("Day 1 (part 1)", Day1.run_part1, data_dir)
    run_bit("Day 1 (part 2)", Day1.run_part2, data_dir)
end

function run_bit(name::String, func::Function, data_dir::String)
    printstyled("$name:", bold=true, color=:green)
    println("\t\t$(func(data_dir))")
end

end # module
