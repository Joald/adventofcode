require 'set'
def input filename = "test.txt"
  f = File.open(filename)
  return f.readlines.map(&:chomp)
end

class Coord
  attr_accessor :x
  attr_accessor :y

  def initialize(x, y)
    @x = x
    @y = y  
  end
  def to_s 
    return "(#{x}, #{y})"
  end
  def eql? other
    @x == other.x and @y == other.y
  end
  def hash 
    [@x,@y].hash
  end
  def off xoff, yoff
    Coord.new x + xoff, y + yoff
  end
end

class Bounds
  attr_accessor :mins
  attr_accessor :maxs
  def initialize(minx, miny, maxx, maxy)
    @mins = Coord.new(minx, miny)
    @maxs = Coord.new(maxx, maxy)
  end
  def to_s 
    return "#{mins} upto #{maxs}"
  end
  def has xy
    mins.x <= xy.x and xy.x <= maxs.x and mins.y <= xy.y and xy.y <= maxs.y
  end
end

def contains coord, blizzards, t, rectsize
  blizzards[0][coord.y].include? 1 + (coord.x - 1 + t) % rectsize.x or
  blizzards[1][coord.y].include? 1 + (coord.x - 1 - t) % rectsize.x or
  blizzards[2][coord.x].include? 1 + (coord.y - 1 - t) % rectsize.y or
  blizzards[3][coord.x].include? 1 + (coord.y - 1 + t) % rectsize.y
end

def main
  lines = input
  part = 2
  
  blizzards = [
    Array.new(lines.length),
    Array.new(lines.length),
    Array.new(lines[0].length), 
    Array.new(lines[0].length)
  ].map do |arr|
    arr.map {|_| Set.new}
  end
  puts blizzards[0][0].length
  bounds = Bounds.new 0, 0, lines[0].length - 1, lines.length - 1
  
  lines.each_with_index do |line, y|
    line.chars.each_with_index do |c, x|
      if "<>v^".include? c then
        blizzards["<>v^".index(c)][if "<>".include? c then y else x end]
          .add(if "<>".include? c then x else y end)
      end
    end
  end
  
  start = Coord.new(1, 0)
  endd = Coord.new(lines[-1].length - 2, lines.length - 1)
  targets = if part == 1 then [endd] else [
    endd,
    start,
    endd,
  ] end
  target = 0
  rectsize = Coord.new lines[0].length - 2, lines.length - 2
  cyc_len = rectsize.x.lcm rectsize.y
  t = 0
  olds = [[start, 0]].to_h
  
  news = [].to_h
  while true
    t += 1
    olds.each do |coord, time|
      if not contains coord, blizzards, t, rectsize and time < cyc_len 
        news[coord] = time + 1
      end
    end
    found = false
    olds.each do |coord, _|
      [[-1, 0], [1, 0], [0, 1], [0, -1]].each do |xoff, yoff|
        pos = coord.off xoff, yoff
        if pos.eql? targets[target] then
          found = true
          break
        end
        if bounds.has pos and lines[pos.y][pos.x] != '#' and
          not contains pos, blizzards, t, rectsize and
          not news.has_key? pos then
          news[pos] = 0
        end
      end
    end
    if found then
      news = [[targets[target], 0]]
      target += 1
      if target == targets.length then
        break
      end
    end
    olds = news
    news = [].to_h
  end
  
  puts t
end

if __FILE__ == $0
  main
end