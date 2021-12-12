import std/os
import std/strformat

import day1

let dir = paramStr(1)


try:
    echo &"Answer to day 1 part 1: {runDay1Part1(dir)}"
except IOError:
    echo &"Couldn't open data at {dir}"
    quit(1)


try:
    echo &"Answer to day 1 part 2: {runDay1Part2(dir)}"
except IOError:
    echo &"Couldn't open data at {dir}"
    quit(1)
