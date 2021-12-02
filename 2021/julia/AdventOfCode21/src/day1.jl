module Day1

using Base.Iterators

mutable struct Scanner
    prev::Int64
    count::Int64
end

struct WindowIterator{I}
    iter::I
    size::Integer
end

function Base.iterate(w::WindowIterator{I}) where I
    result = iterate(w.iter)
    if result == nothing
        return nothing
    end
    (e, state) = result
    len = min(w.size, length(w.iter))
    if len == 0
        return nothing
    end
    window = Vector{eltype(w.iter)}(undef, len)
    window[1] = e
    for i in 2:length(window)
        (e, state) = iterate(w.iter, state)
        window[i] = e
    end
    (window, (window, state))
end

function Base.iterate(w::WindowIterator{I}, state) where I
    (window, state) = state
    result = iterate(w.iter, state)
    if result == nothing
        return nothing
    else
        (next_elem, state) = result
        window = vcat(window[2:w.size], [next_elem])
        return (window, (window, state))
    end
end

Base.IteratorSize(::WindowIterator{I}) where I = Base.HasLength()
Base.IteratorEltype(::WindowIterator{I}) where I = Base.HasEltype()
Base.eltype(w::WindowIterator{I}) where I = Vector{eltype(I)}
Base.length(w::WindowIterator{I}) where I = length(w.iter) == 0 ? 0 : max(length(w.iter) - w.size + 1, 1)

window(iter::I, size::Integer) where I = WindowIterator(iter, size)

cmp(s::Scanner, value::Int64) = s.prev < value
function tick!(s::Scanner, value::Int64)
    if cmp(s, value)
        s.count += 1
    end
    s.prev = value
    s
end

function num_increases(itr)
    (first, rest) = Iterators.peel(itr)
    scan = Scanner(first, 0)
    for x in rest
        tick!(scan, x)
    end
    scan.count
end

function read(f::String)::Array{Int64}
    map(x -> parse(Int64, x), readlines(f))
end

run_part1(data_dir::String) = joinpath(data_dir, "day1.txt") |> read |> num_increases

run_part2(data_dir::String) = joinpath(data_dir, "day1.txt") |> 
    read |> 
    arr -> window(arr, 3) |>
    x -> Iterators.map(sum, x) |>
    num_increases
end
