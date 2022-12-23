function parseMovement(movement)
  # print("mvmnt = '$movement'")
  res = Union{Int64,Bool}[]
  dirs = findall(x -> x == 'L' || x == 'R', movement)
  prev = 1
  for dirindex in dirs
    # print(dirs)
    push!(res, parse(Int64, movement[prev:dirindex-1]))
    push!(res, movement[dirindex] == 'R')
    prev = dirindex + 1
  end
  push!(res, parse(Int64, movement[prev:length(movement)]))
  res
end

function moveBy(board, pos, count)
  # x = 8, y = 144, dir = 4, count = 15
  if pos.dir == 1 # right
    hash = findnext(==('#'), board[pos.x], pos.y)
    spc = findnext(==(' '), board[pos.x], pos.y)
    if hash !== nothing && hash <= pos.y + count
      merge(pos, (y=hash - 1,))
    elseif spc <= pos.y + count
      delta = pos.y + count - spc
      firsthash = findfirst(==('#'), board[pos.x])
      firstdot = findfirst(==('.'), board[pos.x])
      if firsthash === nothing
        merge(pos, (y=firstdot + delta % (spc - firstdot),))
      elseif firstdot < firsthash
        merge(pos, (y=min(firstdot + delta, firsthash - 1),))
      else
        merge(pos, (y=findlast(==('.'), board[pos.x]),))
      end
    else
      merge(pos, (y=pos.y + count,))
    end
  elseif pos.dir == 2 # down
    hash = findnext(l -> l[pos.y] == '#', board, pos.x)
    spc = findnext(l -> l[pos.y] == ' ', board, pos.x)
    if hash !== nothing && hash <= pos.x + count
      merge(pos, (x=hash - 1,))
    elseif spc <= pos.x + count
      delta = pos.x + count - spc
      firsthash = findfirst(l -> l[pos.y] == '#', board)
      firstdot = findfirst(l -> l[pos.y] == '.', board)
      if firsthash === nothing
        merge(pos, (x=firstdot + delta % (spc - firstdot),))
      elseif firstdot < firsthash
        merge(pos, (x=min(firstdot + delta, firsthash - 1),))
      else
        merge(pos, (x=findlast(l -> l[pos.y] == '.', board),))
      end
    else
      merge(pos, (x=pos.x + count,))
    end
  elseif pos.dir == 3 # left
    hash = findprev(==('#'), board[pos.x], pos.y)
    spc = findprev(==(' '), board[pos.x], pos.y)
    if hash !== nothing && hash >= pos.y - count
      merge(pos, (y=hash + 1,))
    elseif spc >= pos.y - count
      delta = count - (pos.y - spc)
      firsthash = findlast(==('#'), board[pos.x])
      firstdot = findlast(==('.'), board[pos.x])
      if firsthash === nothing
        merge(pos, (y=firstdot - delta % (firstdot - spc),))
      elseif firstdot > firsthash
        merge(pos, (y=max(firstdot - delta, firsthash + 1),))
      else # ∘
        merge(pos, (y=findfirst(==('.'), board[pos.x]),))
      end
    else
      merge(pos, (y=pos.y - count,))
    end
  elseif pos.dir == 4 # up
    hash = findprev(l -> l[pos.y] == '#', board, pos.x)
    spc = findprev(l -> l[pos.y] == ' ', board, pos.x)
    if hash !== nothing && hash >= pos.x - count
      merge(pos, (x=hash + 1,))
    elseif spc >= pos.x - count
      delta = count - (pos.x - spc)
      firsthash = findlast(l -> l[pos.y] == '#', board)
      firstdot = findlast(l -> l[pos.y] == '.', board)
      # if pos.x == 8 && pos.y == 144
      #   println((delta=delta, firsthash=firsthash, firstdot=firstdot), firstdot - delta)
      # end
      if firsthash === nothing
        merge(pos, (x=firstdot - delta % (firstdot - spc),))
      elseif firstdot > firsthash
        merge(pos, (x=max(firstdot - delta, firsthash + 1),))
      else # ∘
        merge(pos, (x=findfirst(l -> l[pos.y] == '.', board),))
      end
    else
      merge(pos, (x=pos.x - count,))
    end
  end
end

mutable struct Side
  xy::NamedTuple{(:x, :y),Tuple{Int64,Int64}}
  gridxy::Tuple{Int64,Int64}
  up
  updir # which dir you travel after moving up
  left
  leftdir
  right
  rightdir
  down
  downdir
end

function moveBy2(board, pos, count)
  if pos.dir == 1 # right
    hash = findnext(==('#'), board[pos.x], pos.y)
    spc = findnext(==(' '), board[pos.x], pos.y)
    if hash !== nothing && hash <= pos.y + count
      merge(pos, (y=hash - 1,))
    elseif spc <= pos.y + count
      delta = pos.y + count - spc
      firsthash = findfirst(==('#'), board[pos.x])
      firstdot = findfirst(==('.'), board[pos.x])
      if firsthash === nothing
        merge(pos, (y=firstdot + delta % (spc - firstdot),))
      elseif firstdot < firsthash
        merge(pos, (y=min(firstdot + delta, firsthash - 1),))
      else
        merge(pos, (y=findlast(==('.'), board[pos.x]),))
      end
    else
      merge(pos, (y=pos.y + count,))
    end
  elseif pos.dir == 2 # down
    hash = findnext(l -> l[pos.y] == '#', board, pos.x)
    spc = findnext(l -> l[pos.y] == ' ', board, pos.x)
    if hash !== nothing && hash <= pos.x + count
      merge(pos, (x=hash - 1,))
    elseif spc <= pos.x + count
      delta = pos.x + count - spc
      firsthash = findfirst(l -> l[pos.y] == '#', board)
      firstdot = findfirst(l -> l[pos.y] == '.', board)
      if firsthash === nothing
        merge(pos, (x=firstdot + delta % (spc - firstdot),))
      elseif firstdot < firsthash
        merge(pos, (x=min(firstdot + delta, firsthash - 1),))
      else
        merge(pos, (x=findlast(l -> l[pos.y] == '.', board),))
      end
    else
      merge(pos, (x=pos.x + count,))
    end
  elseif pos.dir == 3 # left
    hash = findprev(==('#'), board[pos.x], pos.y)
    spc = findprev(==(' '), board[pos.x], pos.y)
    if hash !== nothing && hash >= pos.y - count
      merge(pos, (y=hash + 1,))
    elseif spc >= pos.y - count
      delta = count - (pos.y - spc)
      firsthash = findlast(==('#'), board[pos.x])
      firstdot = findlast(==('.'), board[pos.x])
      if firsthash === nothing
        merge(pos, (y=firstdot - delta % (firstdot - spc),))
      elseif firstdot > firsthash
        merge(pos, (y=max(firstdot - delta, firsthash + 1),))
      else # ∘
        merge(pos, (y=findfirst(==('.'), board[pos.x]),))
      end
    else
      merge(pos, (y=pos.y - count,))
    end
  elseif pos.dir == 4 # up
    hash = findprev(l -> l[pos.y] == '#', board, pos.x)
    spc = findprev(l -> l[pos.y] == ' ', board, pos.x)
    if hash !== nothing && hash >= pos.x - count
      merge(pos, (x=hash + 1,))
    elseif spc >= pos.x - count
      delta = count - (pos.x - spc)
      firsthash = findlast(l -> l[pos.y] == '#', board)
      firstdot = findlast(l -> l[pos.y] == '.', board)
      # if pos.x == 8 && pos.y == 144
      #   println((delta=delta, firsthash=firsthash, firstdot=firstdot), firstdot - delta)
      # end
      if firsthash === nothing
        merge(pos, (x=firstdot - delta % (firstdot - spc),))
      elseif firstdot > firsthash
        merge(pos, (x=max(firstdot - delta, firsthash + 1),))
      else # ∘
        merge(pos, (x=findfirst(l -> l[pos.y] == '.', board),))
      end
    else
      merge(pos, (x=pos.x - count,))
    end
  end
end

function getat(grid, maybexy)
  !isnothing(maybexy) && grid[maybexy...]
end

function connectgrid!(grid)
  for i in 1:size(grid)[1]-1
    for j in 1:size(grid)[2]-1
      if grid[i, j] === nothing
        continue
      end
      if grid[i+1, j] !== nothing
        grid[i, j].down = (i + 1, j)
        grid[i, j].downdir = 2
        grid[i+1, j].up = (i, j)
        grid[i+1, j].updir = 4
      end
      if grid[i, j+1] !== nothing
        grid[i, j].right = (i, j + 1)
        grid[i, j].rightdir = 1
        grid[i, j+1].left = (i, j)
        grid[i, j+1].leftdir = 3
      end
    end
  end

  for _ in 1:1
    for i in 1:size(grid)[1]
      for j in 1:size(grid)[2]
        if grid[i, j] === nothing
          continue
        end
        cur = grid[i, j]
        left = getat(grid, cur.left)
        right = getat(grid, cur.right)
        up = getat(grid, cur.up)
        down = getat(grid, cur.down)
        if left isa Side
          lup = getat(grid, left.up)
          if lup isa Side && isnothing(cur.up)
            cur.up = lup.gridxy
            cur.updir = 3
            lup.right = cur.gridxy
            lup.rightdir = 2
          end
          ldown = getat(grid, left.down)
          if ldown isa Side && isnothing(cur.down)
            cur.down = ldown.gridxy
            cur.downdir = 3
            ldown.right = cur.gridxy
            ldown.rightdir = 4
          end
        end
        if right isa Side
          rup = getat(grid, right.up)
          if rup isa Side && isnothing(cur.up)
            cur.up = rup.gridxy
            cur.updir = 1
            rup.left = cur.gridxy
            rup.leftdir = 2
          end
          rdown = getat(grid, right.down)
          if rdown isa Side && isnothing(cur.down)
            cur.down = rdown.gridxy
            cur.downdir = 1
            rdown.left = cur.gridxy
            rdown.leftdir = 4
          end
        end
        if up isa Side
          upl = getat(grid, up.left)
          if upl isa Side && isnothing(cur.left)
            cur.left = upl.gridxy
            cur.leftdir = 4
            upl.down = cur.gridxy
            upl.downdir = 1
          end
          upr = getat(grid, up.right)
          if upr isa Side && isnothing(cur.right)
            cur.right = upr.gridxy
            cur.rightdir = 4
            upr.down = cur.gridxy
            upr.downdir = 3
          end
        end
        if down isa Side
          downl = getat(grid, down.left)
          if downl isa Side && isnothing(cur.left)
            cur.left = downl.gridxy
            cur.leftdir = 2
            downl.up = cur.gridxy
            downl.updir = 1
          end
          downr = getat(grid, down.right)
          if downr isa Side && isnothing(cur.right)
            cur.right = downr.gridxy
            cur.rightdir = 2
            downr.up = cur.gridxy
            downr.updir = 3
          end
        end
      end
    end
  end
end

function manuallycorrectgrid!(grid)
  grid[1, 2].up = (4, 1)
  grid[1, 2].updir = 1
  grid[1, 2].left = (3, 1)
  # grid[1, 2].leftdir = 
  # grid[1, 2].
end

function part1(lines)
  movement = parseMovement(lines[length(lines)])
  lines = lines[1:length(lines)-2]
  w = maximum(length, lines)
  padding = " "^w
  pushfirst!(lines, padding)
  push!(lines, padding)
  lines = map(s -> ' ' * s * ' ' * ' '^(w - length(s)), lines)
  # grid = Array{Union{Side,Nothing}}(nothing, length(lines) ÷ 50, w ÷ 50)
  # for i in 1:size(grid)[1]
  #   for j in 1:size(grid)[2]
  #     x = (i - 1) * 50 + 2
  #     y = (j - 1) * 50 + 2
  #     if lines[x][y] != ' '
  #       grid[i, j] = Side((x=x, y=y), (i, j), nothing, nothing, nothing, nothing, nothing, nothing, nothing, nothing)
  #     end
  #   end
  # end
  # connectgrid!(grid)
  # display(grid)
  # manuallycorrectgrid!(grid)
  # display(grid)
  # return
  drawing = collect.(deepcopy(lines))



  # x, y, dir
  # confusingly, xs change vertically and ys horizontally
  # dir: -> = 1, v = 2, <- = 3, ^ = 4
  pos = (x=2, y=findfirst(==('.'), lines[2]), dir=1)
  for move in movement
    pos = if move isa Bool
      move ? merge(pos, (dir=pos.dir + 1,)) : merge(pos, (dir=pos.dir - 1,))
    else # move isa Int64
      # println(pos)
      # drawingg = deepcopy(drawing)
      # drawingg[pos.x][pos.y] = 'X'
      # println(join(map(l -> join(l, ""), drawingg), "\n"))
      # println("moving by $move")
      drawing[pos.x][pos.y] = ">v<^"[pos.dir]

      moveBy(lines, pos, move)
    end
    if pos.dir < 1
      pos = merge(pos, (dir=4,))
    elseif pos.dir > 4
      pos = merge(pos, (dir=1,))
    end
    if lines[pos.x][pos.y] != '.'
      println("ERROR", pos)
      drawingg = deepcopy(drawing)
      drawingg[pos.x][pos.y] = 'X'
      println(join(map(l -> join(l, ""), drawingg), "\n"))

      return
    end
  end
  println(join(map(l -> join(l, ""), drawing), "\n"))
  merge(pos, (res=(pos.x - 1) * 1000 + (pos.y - 1) * 4 + (pos.dir - 1),))
end

mutable struct Face
  upi
  dni
  lti
  rti
  minx
  maxx
  miny
  maxy
  updir
  dndir
  ltdir
  rtdir
  uprev
  dnrev
  ltrev
  rtrev
end

#  56
#  4
# 23
# 1
#

# function getfaces()
# fcs = Array{Face}[]
# push!(fcs, Face(2, 6, 5, 3, 101, 200, 1, 50, 4, 2, 2, 4, false, ))
# end

function part2(lines)
  movement = parseMovement(lines[length(lines)])
  lines = lines[1:length(lines)-2]
  w = maximum(length, lines)
  padding = " "^(w + 2)
  lines = map(s -> ' ' * s * ' ' * ' '^(w - length(s)), lines)
  push!(lines, padding)
  pushfirst!(lines, padding)
  tp = Dict{Tuple{Int64,Int64,Int64},Tuple{Int64,Int64,Int64}}()
  for i in 1:50
    tp[(0 + 1, i + 50 + 1, 4)] = (150 + i + 1, 1 + 1, 1)      # 5u -> 1l
    tp[(0 + 1, i + 100 + 1, 4)] = (200 + 1, i + 1, 4)         # 6u -> 1d
    tp[(i + 1, 50 + 1, 3)] = (151 - i + 1, 1 + 1, 1)          # 5l -> 2l -
    tp[(i + 1, 151 + 1, 1)] = (151 - i + 1, 100 + 1, 3)       # 6r -> 3r -
    tp[(51 + 1, 100 + i + 1, 2)] = (50 + i + 1, 100 + 1, 3)   # 6d -> 4r
    tp[(50 + i + 1, 50 + 1, 3)] = (101 + 1, i + 1, 2)         # 4l -> 2u
    tp[(50 + i + 1, 101 + 1, 1)] = (50 + 1, 100 + i + 1, 4)   # 4r -> 6d
    tp[(100 + 1, i + 1, 4)] = (50 + i + 1, 51 + 1, 1)         # 2u -> 4l
    tp[(100 + i + 1, 0 + 1, 3)] = (51 - i + 1, 51 + 1, 1)     # 2l -> 5l -
    tp[(100 + i + 1, 101 + 1, 1)] = (51 - i + 1, 150 + 1, 3)  # 3r -> 6r -
    tp[(151 + 1, 50 + i + 1, 2)] = (150 + i + 1, 50 + 1, 3)   # 3d -> 1r
    tp[(150 + i + 1, 0 + 1, 3)] = (1 + 1, 50 + i + 1, 2)      # 1l -> 5u
    tp[(150 + i + 1, 51 + 1, 1)] = (150 + 1, 50 + i + 1, 4)   # 1r -> 3d
    tp[(201 + 1, i + 1, 2)] = (1 + 1, 100 + i + 1, 2)         # 1d -> 6u
  end
  display(tp)
  pos = (2, findfirst(==('.'), lines[2]), 1)

  drawing = collect.(deepcopy(lines))

  # interactive edge exploration:

  # drawing[pos[1]][pos[2]] = 'X'
  # println(join(map(l -> join(l, ""), drawing), "\n"))
  # drawing[pos[1]][pos[2]] = '.'


  # while true
  #   cmd = read(stdin, Char)
  #   x, y, dir = pos
  #   if cmd == 'w'
  #     x -= 1
  #     dir = 4
  #   elseif cmd == 's'
  #     x += 1
  #     dir = 2
  #   elseif cmd == 'a'
  #     y -= 1
  #     dir = 3
  #   elseif cmd == 'd'
  #     y += 1
  #     dir = 1
  #   end
  #   pos = (x, y, dir)
  #   if cmd == ' '
  #     pos = tp[pos]
  #   end

  #   display(pos)
  #   bkp = drawing[x][y]
  #   drawing[x][y] = 'X'
  #   println(join(map(l -> join(l, ""), drawing), "\n"))
  #   drawing[x][y] = bkp
  # end
  # return

  for move in movement
    x, y, dir = pos
    drawing[x][y] = ">v<^"[dir]
    if move isa Bool
      println("turning $(move ? 'R' : 'L')")
      ndir = move ? dir + 1 : dir - 1
      if ndir < 1
        ndir = 4
      elseif ndir > 4
        ndir = 1
      end
      pos = (x, y, ndir)
    else
      println("moving by $move, pos=$pos")
      cnt = move
      while cnt > 0
        x, y, dir = pos
        drawing[x][y] = ">v<^"[dir]
        nx = dir == 2 ? x + 1 : dir == 4 ? x - 1 : x
        ny = dir == 1 ? y + 1 : dir == 3 ? y - 1 : y
        if lines[nx][ny] == '.'
          pos = (nx, ny, dir)
        elseif lines[nx][ny] == ' '
          npos = tp[(nx, ny, dir)]
          if lines[npos[1]][npos[2]] == '#'
            break
          else
            pos = npos
          end
        else
          break
        end
        cnt -= 1
      end
      drawing[pos[1]][pos[2]] = ">v<^"[dir]
      println(join(map(l -> join(l, ""), drawing), "\n"))
    end
  end
  println(join(map(l -> join(l, ""), drawing), "\n"))
  x, y, dir = pos
  (pos, (res=(x - 1) * 1000 + (y - 1) * 4 + (dir - 1),))
end

teststr = """
        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"""
testing = false

lines = if testing
  split(teststr, "\n")
else
  readlines(open("input.txt"))
end
print(part2(lines))

# part 2 1st try 75304 (too low)
# part 2 2nd try (188, 37, 2), (res = 187145,) (high)
# part 3 3rd try (131, 18, 1), (res = 130068,)