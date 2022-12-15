fs = require 'fs'

# https://coffeescript-cookbook.github.io/chapters/arrays/zip-function
zip = () ->
  lengthArray = (arr.length for arr in arguments)
  length = Math.min(lengthArray...)
  for i in [0...length]
    arr[i] for arr in arguments

testStr = """Sensor at x=2, y=18: closest beacon is at x=-2, y=15
             Sensor at x=9, y=16: closest beacon is at x=10, y=16
             Sensor at x=13, y=2: closest beacon is at x=15, y=3
             Sensor at x=12, y=14: closest beacon is at x=10, y=16
             Sensor at x=10, y=20: closest beacon is at x=10, y=16
             Sensor at x=14, y=17: closest beacon is at x=10, y=16
             Sensor at x=8, y=7: closest beacon is at x=2, y=10
             Sensor at x=2, y=0: closest beacon is at x=2, y=10
             Sensor at x=0, y=11: closest beacon is at x=2, y=10
             Sensor at x=20, y=14: closest beacon is at x=25, y=17
             Sensor at x=17, y=20: closest beacon is at x=21, y=22
             Sensor at x=16, y=7: closest beacon is at x=15, y=3
             Sensor at x=14, y=3: closest beacon is at x=15, y=3
             Sensor at x=20, y=1: closest beacon is at x=15, y=3"""

testing = no



part12 = (data, part) ->
  
  bigg = 10000000000
  target = if testing then 10 else 2000000
  mark = (x, y) ->
    if part == 1
      marked.add(x+ bigg * y) if y == target
  lines = data.split '\n'
  parse = (s) -> 
    s = s.split('x=')[1].split ', y='
    (parseInt i for i in s)
  
  sensors = ((parse half for half in line.split ':') for line in lines when line isnt "")
  [minx, miny, maxx, maxy] = [Infinity, Infinity, -Infinity, -Infinity]
  bcns = new Set()
  for sensor in sensors
    bcns.add(sensor[1][0]) if sensor[1][1] == target
    for coords in sensor
      [minx, miny] = (Math.min pair... for pair in zip coords, [minx, miny])
      [maxx, maxy] = (Math.max pair... for pair in zip coords, [maxx, maxy])

  if part == 2
    maxcoord = if testing then 20 else 4000000

    for i in [0..sensors.length - 1]
      [sx, sy] = sensors[i][0]
      [bcx, bcy] = sensors[i][1]
      sbdist = Math.abs(sx - bcx) + Math.abs(sy - bcy)
      sensors[i] = [sx, sy, sbdist]

    for x in [0..maxcoord]
      console.log x if x % 10000 == 0
      y = 0
      while y <= maxcoord
        all = yes
        mindist = Infinity
        for [sx, sy, dist] in sensors
          sdist = Math.abs(sx - x) + Math.abs(sy - y)
          # console.log sx, sy, sdist, dist if x == 14 and y == 11
          all &&= sdist > dist
          mindist = Math.min(mindist, sdist - dist)
        return [x, y] if all

        y += Math.max(1, Math.abs(mindist))

  else 
    minx -= 10
    miny -= 10
    maxx += 10
    maxy += 10
    print = (m) ->
      console.log(minx, maxx, miny, maxy)
      line = ''.padEnd 4, ' '
      for i in [minx..maxx]
        line += if i % 5 == 0 then Math.abs(Math.floor(i // 10)) else ' '
      console.log(line)
      
      line = ''.padEnd 4, ' '
      for i in [minx..maxx]
        line += if i % 10 == 0 then '0' else if i % 5 == 0 then '5' else ' '
      console.log(line)
      for j in [miny..maxy]
        s = '' + j + ':'
        s = s.padEnd 4, ' '
        for i in [minx..maxx]
          val = i + bigg * j
          s += if sns.has val then 'S' else if bcns.has val then 'B' else if m.has val then '#' else '.'
        console.log s
  
    marked = new Set()
    cnt = 0
    total = 0
    for sensor in sensors
      [sx, sy] = sensor[0]
      [bcx, bcy] = sensor[1]
      sbdist = Math.abs(sx - bcx) + Math.abs(sy - bcy)
      targetydist = Math.abs(sy - target)
      continue if targetydist > sbdist
      free = sbdist - targetydist
      # console.log "sx =", sx, "sy =", sy, "bcx =", bcx, "bcy =", bcy, "sbdist =", sbdist, "targetydist =", targetydist, "free =", free
      for x in [sx - free..sx + free]
        marked.add x if !bcns.has x
    marked.size


fs.readFile 'input.txt', 'utf8', (err, data) ->
  if err
    console.error err
  else
    console.log part12 (if testing then testStr else data), 2
  x = new Set()
