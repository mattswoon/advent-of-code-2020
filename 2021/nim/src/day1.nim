import std/os
import std/sequtils
import strutils


proc updateCount(value, prev: int, count: var int) =
    if (prev >= 0) and (value > prev):
        count = count + 1


proc runDay1Part1*(dir: string): int {. raises: [IOError].}=
    let path = joinPath(dir, "day1.txt")
    result = 0
    var prev = -1
    for line in readFile(path).splitLines():
        try:
            let value = parseInt(line)
            updateCount(value, prev, result)
            prev = value
        except ValueError:
            continue


iterator iterInts(f: string): int =
    for line in readFile(f).splitLines():
        try:
            yield parseInt(line)
        except ValueError:
            continue


proc readData(f: string): seq[int] =
    toSeq(iterInts(f))


proc sum(x: array[3, int]): int =
    x.foldl(a + b)


proc runDay1Part2*(dir: string): int {. raises: [IOError] .} =
    let path = joinPath(dir, "day1.txt")
    let data = readData(path)
    var prev = [data[0], data[1], data[2]]
    result = 0
    for x in data[3..^1]:
        let cur = [prev[1], prev[2], x]
        if sum(cur) > sum(prev):
            result += 1
        prev = cur
