module AdventOfCode21

include("./day1.jl")

function run(data_dir::String)
    printstyled("Day 1:", bold=true, color=:green)
    print("\t\t$(Day1.run(data_dir))")
end

end # module
