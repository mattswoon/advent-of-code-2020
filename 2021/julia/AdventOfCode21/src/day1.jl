module Day1

mutable struct Scanner
    prev::Int64
    count::Int64
end

cmp(s::Scanner, value::Int64) = s.prev < value
function tick!(s::Scanner, value::Int64)
    if cmp(s, value)
        s.count += 1
    end
    s.prev = value
    s
end

function num_increases(data::Array{Int64})::Int64 
    scan = Scanner(first(data), 0)
    for x in data[2:end]
        tick!(scan, x)
    end
    scan.count
end

function read(f::String)::Array{Int64}
    map(x -> parse(Int64, x), readlines(f))
end

run(data_dir::String) = joinpath(data_dir, "day1.txt") |> read |> num_increases

end
