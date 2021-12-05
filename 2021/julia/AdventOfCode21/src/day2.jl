module Day2

abstract type Instruction end

struct Forward <: Instruction
    x::Int64
end

struct Down <: Instruction
    x::Int64
end

struct Up <: Instruction
    x::Int64
end

function instruction(s::String)
    (cmd, num) = split(s, " ")
    num = parse(Int64, num)
    if cmd == "forward"
        return Forward(num)
    end
    if cmd == "up"
        return Up(num)
    end
    if cmd == "down"
        return Down(num)
    end
    error("Bad instruction")
end

struct Position
    horizontal::Int64
    depth::Int64
end

Position() = Position(0, 0)

Base.:+(up::Up, pos::Position) = Position(pos.horizontal, pos.depth - up.x)
Base.:+(down::Down, pos::Position) = Position(pos.horizontal, pos.depth + down.x)
Base.:+(fwd::Forward, pos::Position) = Position(pos.horizontal + fwd.x, pos.depth)

read(f::String) = map(instruction, readlines(f))

run_part1(data_dir::String) = joinpath(data_dir, "day2.txt") |> 
    read |>
    arr -> foldr(+, reverse(arr); init=Position()) |>
    pos -> pos.horizontal * pos.depth

struct PositionWithAim
    pos::Position
    aim::Int64
end

PositionWithAim() = PositionWithAim(Position(), 0)

Base.:+(up::Up, p::PositionWithAim) = PositionWithAim(p.pos, p.aim - up.x)
Base.:+(down::Down, p::PositionWithAim) = PositionWithAim(p.pos, p.aim + down.x)
Base.:+(fwd::Forward, p::PositionWithAim) = PositionWithAim(Down(p.aim * fwd.x) + (fwd + p.pos), p.aim)

run_part2(data_dir::String) = joinpath(data_dir, "day2.txt") |> 
    read |>
    arr -> foldr(+, reverse(arr); init=PositionWithAim()) |>
    pos -> pos.pos.horizontal * pos.pos.depth

end
