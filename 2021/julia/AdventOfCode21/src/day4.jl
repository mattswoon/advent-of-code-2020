module Day4

mutable struct BingoBoard
    x::Array{Int64, 2}
    checked::Array{Bool, 2}
end

function Base.parse(::Type{BingoBoard}, s)
    s = strip(s)
    b = Array{Int64, 2}(undef, 5, 5)
    checked = zeros(Bool, 5, 5)
    for (i, line) in enumerate(split(s, "\n"))
        row = split(line, " ") |>
            x -> filter(!isempty, x) |> 
            x -> parse.(Int64, x)
        b[i, :] = row
    end
    BingoBoard(b, checked)
end

function read(f::String)
    s = Base.read(f, String) |> s -> split(s, "\n\n")
    numbers = popfirst!(s) |> 
        a -> split(a, ",") |>
        a -> parse.(Int64, a)
    boards = parse.(BingoBoard, s)
    (boards, numbers)
end

function mark!(board::BingoBoard, value::Int64)
    board.checked = board.checked .| (board.x .== value)
    board
end

check(board::BingoBoard) = (sum(board.checked, dims=1) |> a -> any(x -> x == 5, a)) |
    (sum(board.checked, dims=2) |> a -> any(x -> x == 5, a))

score(board::BingoBoard, last::Int64) = sum(.!(board.checked) .* board.x) * last

function first_winning_board!(boards::Vector{BingoBoard}, numbers::Vector{Int64})
    for num in numbers
        mark!.(boards, num)
        idx = findfirst(check, boards)
        if idx != nothing
            return (boards[idx], num)
        end
    end
    nothing
end

run_part1(data_dir::String) = joinpath(data_dir, "day4.txt") |>
    read |>
    a -> first_winning_board!(a...) |>
    a -> score(a...)

function last_winning_board!(boards::Vector{BingoBoard}, numbers::Vector{Int64})
    for num in numbers
        mark!.(boards, num)
        if (length(boards) == 1) && check(first(boards))
            return (first(boards), num)
        end
        findall(check, boards) |> idxs -> deleteat!(boards, idxs)
    end
    nothing
end

run_part2(data_dir::String) = joinpath(data_dir, "day4.txt") |>
    read |>
    a -> last_winning_board!(a...) |>
    a -> score(a...)

end # module
