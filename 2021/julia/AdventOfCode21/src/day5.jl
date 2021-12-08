module Day5

TESTINPUT = """0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"""

struct Coord
    x::Int64
    y::Int64
end

Base.:+(a::Coord, b::Coord) = Coord(a.x + b.x, a.y + b.y)
Base.:-(a::Coord, b::Coord) = Coord(a.x - b.x, a.y - b.y)

Coord(s) = split(s, ",") |> 
    x -> parse.(Int64, x) |> 
    x -> Coord(x...)

struct Line
    from::Coord
    to::Coord
end

is_vertical_or_horizontal(l::Line) = l.from.x == l.to.x || l.from.y == l.to.y
gradient(l::Line) = (l.to.y - l.from.y) // (l.to.x - l.from.x)
x_direction(l::Line) = sign(l.to.x - l.from.x)
is_coord_on_line(c::Coord, l::Line) = (l.to.x - l.from.x) * (c.y - l.from.y) == (l.to.y - l.from.y) * (c.x - l.from.x)
function step(l::Line)
    m = gradient(l)
    if m.num == 0
        Coord(sign(l.to.x - l.from.x), 0)
    elseif m.den == 0
        Coord(0, sign(l.to.y - l.from.y))
    else
        Coord(x_direction(l) * m.den, x_direction(l) * m.num)
    end
end

function num_steps(l::Line)
    s = step(l)
    if s.x == 0
        Int64((l.to.y - l.from.y) / s.y) + 1
    else
        Int64((l.to.x - l.from.x) / s.x) + 1
    end
end

Base.iterate(l::Line) = (l.from, l.from + step(l))
Base.iterate(l::Line, c::Coord) = c == (l.to + step(l)) ? nothing : (c, c + step(l))
Base.IteratorSize(::Type{Line}) = Base.HasLength()
Base.IteratorEltype(::Type{Line}) = Base.HasEllype()
Base.eltype(l::Line) = Coord
Base.length(l::Line) = num_steps(l)

min_x(l::Line) = min(l.from.x, l.to.x)
max_x(l::Line) = max(l.from.x, l.to.x)
min_y(l::Line) = min(l.from.y, l.to.y)
max_y(l::Line) = max(l.from.y, l.to.y)

BBox = Tuple{Int64, Int64, Int64, Int64}
bbox(l::Line) = (min_x(l), min_y(l), max_x(l), max_y(l))
bigger_bbox(left::BBox, right::BBox) = (min(left[1], right[1]), min(left[2], right[2]), max(left[3], right[3]), max(left[4], right[4]))

Line(s) = split(s, " -> ") |> 
    x -> Coord.(x) |> 
    x -> Line(x...)

mutable struct Diagram
    x::Array{Int64, 2}
    top_left::Coord
    width::Int64
    height::Int64
end

diagram_size(ls::Vector{Line}) = mapreduce(bbox, bigger_bbox, ls)

function Diagram(ls::Vector{Line})
    box = diagram_size(ls)
    top_left = Coord(box[1], box[2])
    width = box[3] - box[1] + 1
    height = box[4] - box[2] + 1
    x = zeros(Int64, height, width)
    Diagram(x, top_left, width, height)
end

function mark!(d::Diagram, c::Coord)
    col = c.x - d.top_left.x + 1
    row = c.y - d.top_left.y + 1
    d.x[row, col] += 1
    d
end

function mark!(d::Diagram, l::Line)
    for c in l
        mark!(d, c)
    end
    d
end

function mark!(d::Diagram, lines::Vector{Line})
    for line in lines
        mark!(d, line)
    end
    d
end

marked_diagram(lines::Vector{Line}) = Diagram(lines) |> d -> mark!(d, lines)

count_at_least(d::Diagram, x::Int64) = sum(d.x .>= x)

run_part1(data_dir::String) = joinpath(data_dir, "day5.txt") |>
    readlines |>
    ls -> Line.(ls) |>
    ls -> filter(is_vertical_or_horizontal, ls) |>
    marked_diagram |>
    d -> count_at_least(d, 2)

run_part2(data_dir::String) = joinpath(data_dir, "day5.txt") |>
    readlines |>
    ls -> Line.(ls) |>
    marked_diagram |>
    d -> count_at_least(d, 2)

end
