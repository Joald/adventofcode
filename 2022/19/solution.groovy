import groovy.transform.TupleConstructor

def testStr = """Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."""
/*
Blueprint 1:
  Each ore robot costs 4 ore.
  Each clay robot costs 2 ore.
  Each obsidian robot costs 3 ore and 14 clay.
  Each geode robot costs 2 ore and 7 obsidian.

Blueprint 2:
  Each ore robot costs 2 ore.
  Each clay robot costs 3 ore.
  Each obsidian robot costs 3 ore and 8 clay.
  Each geode robot costs 3 ore and 12 obsidian.
*/

def fileContents = new File('input.txt').text
def testing = false

def contents = testing ? testStr : fileContents
def lines = contents.split("\n")

@TupleConstructor()
class Blueprint {
  int number 
  int oreRobotOres
  int clayRobotOres
  int obsidianRobotOres
  int obsidianRobotClay
  int geodeRobotOres
  int geodeRobotObsidian

  def maxCost(Type type) {
    switch(type) {
      case Type.Ore:
        return Math.max(Math.max(oreRobotOres, clayRobotOres), Math.max(obsidianRobotOres, geodeRobotOres))   
      case Type.Clay:
        return obsidianRobotClay
      case Type.Obsidian:
        return geodeRobotObsidian
      case Type.Geode:
        return 0
    }
  }
}

@TupleConstructor(force=true)
class Inventory {
  ResourcePack resources
  ResourcePack bots
  int timeLeft
  def clone() {
    return new Inventory(resources.clone(), bots.clone(), timeLeft)
  }
  def buy(Blueprint bp, Type type) {
    switch(type) {
      case Type.Ore:
        resources.ore -= bp.oreRobotOres
        bots.ore++
        break
      case Type.Clay:
        resources.ore -= bp.clayRobotOres
        bots.clay++
        break
      case Type.Obsidian:
        resources.ore -= bp.obsidianRobotOres
        resources.clay -= bp.obsidianRobotClay
        bots.obsidian++
        break
      case Type.Geode:
        resources.ore -= bp.geodeRobotOres
        resources.obsidian -= bp.geodeRobotObsidian
        bots.geode++
        break
    }
  }
  def endMinute(ResourcePack production) {
    resources.add(production)
    timeLeft--
  }
  def endMinutes(int mins) {
    resources.add(bots.times(mins))
    timeLeft -= mins
  }
  def howLongUntilAffordable(Blueprint bp, Type type) {
    switch (type) {
      case Type.Ore:
        return divCeil(bp.oreRobotOres - resources.ore, bots.ore)
      case Type.Clay:
        return divCeil(bp.clayRobotOres - resources.ore, bots.ore)\
      case Type.Obsidian:
        return Math.max(divCeil(bp.obsidianRobotOres - resources.ore, bots.ore), divCeil(bp.obsidianRobotClay - resources.clay, bots.clay))
      case Type.Geode:
        return Math.max(divCeil(bp.geodeRobotOres - resources.ore, bots.ore), divCeil(bp.geodeRobotObsidian - resources.obsidian, bots.obsidian))
    }
  }
  def divCeil(x, y) {
    return y == 0 ? 30 : (int) Math.ceil((double)x / y)      
  }
  def optimisticGeodes() {
    return resources.geode + bots.geode * timeLeft + (timeLeft * (timeLeft - 1)) / 2
  }
}



@TupleConstructor()
class ResourcePack {
  int ore
  int clay
  int obsidian
  int geode
  def add(ResourcePack other) {
    ore += other.ore;
    clay += other.clay;
    obsidian += other.obsidian;
    geode += other.geode;
  }
  def clone() {
    return new ResourcePack(ore, clay, obsidian, geode)
  }

  def getVal(Type type) {
    switch(type) {
      case Type.Ore: return ore
      case Type.Clay: return clay
      case Type.Obsidian: return obsidian
      case Type.Geode: return geode
    }
  }
  def times(int mul) {
    return new ResourcePack(mul * ore, mul* clay, mul * obsidian, mul * geode)
  }
}

enum Type {
  Ore, Clay, Obsidian, Geode
}

def canAfford(Blueprint bp, Inventory inv, Type type) {
  switch(type) {
    case Type.Ore:
      return bp.oreRobotOres <= inv.resources.ore
    case Type.Clay:
      return bp.clayRobotOres <= inv.resources.ore
    case Type.Obsidian:
      return bp.obsidianRobotOres <= inv.resources.ore 
        && bp.obsidianRobotClay <= inv.resources.clay
    case Type.Geode:
      return bp.geodeRobotOres <= inv.resources.ore
        && bp.geodeRobotObsidian <= inv.resources.obsidian
  }
}

def makesSenseToBuy(Blueprint bp, Inventory inv, Type type) {
  def maxPrice = bp.maxCost(type)
  def leftCheck = maxPrice * (inv.timeLeft - 1) 
    > inv.timeLeft * inv.bots.getVal(type) + inv.resources.getVal(type)
  def throughputCheck = maxPrice > inv.bots.getVal(type)
  return leftCheck && throughputCheck
}

def possiblePurchases(Blueprint bp, Inventory starting) {
  def afford = Type.values().collect { canAfford(bp, starting, it) }
  def invs = []
  if (afford[Type.Geode.ordinal()]) {
    starting.endMinute(starting.bots)
    starting.buy(bp, Type.Geode)
    return [starting]
  }

  for (type in Type.values()) {
    def inv = starting.clone()
    if (!afford[type.ordinal()]) {
      def mins = Math.min(inv.timeLeft, starting.howLongUntilAffordable(bp, type))
      inv.endMinutes(mins)
      if (inv.timeLeft == 0) {
        invs.add(inv)
        continue
      }
    }
    inv.endMinute(inv.bots)
    inv.buy(bp, type)
    invs.add(inv)
  }

  return invs.reverse()
}

def parseBp(line) {
  line = line.split(':')
  // println line[1]
  def num = line[0].split(' ')[1].toInteger()
  // println line[1].tokenize('.')
  line = line[1].tokenize('.')
  // println line
  line = line.collect { it.split(' ') }
  // println line
  return new Blueprint(
    num, 
    line[0][5].toInteger(),
    line[1][5].toInteger(),
    line[2][5].toInteger(),
    line[2][8].toInteger(),
    line[3][5].toInteger(),
    line[3][8].toInteger(),
  )
}

def numOfGeodes(Blueprint bp, int part) {
  def start = new Inventory(
    new ResourcePack(0, 0, 0, 0), 
    new ResourcePack(1, 0, 0, 0),
    part == 1 ? 24 : 32
  )

  ArrayDeque q = new ArrayDeque<Inventory>()
  q.add(start)

  def maxGeodes = 0

  def minTimeLeft = 25

  while (!q.isEmpty()) {
    Inventory inv = q.pollLast()

    def nexts = possiblePurchases(bp, inv)
    for (i in nexts) {
      if (i.resources.geode > maxGeodes) {
        println "new max geodes: ${i.resources.geode}"
      }
      maxGeodes = Math.max(maxGeodes, i.resources.geode)
      if (i.timeLeft > 0 && i.optimisticGeodes() >= maxGeodes) {
        q.addLast(i)
      }
      if (i.timeLeft < 0) {
        println "error!"
      }
    }
  }
  println "bp #${bp.number} got ${maxGeodes} geodes"
  return maxGeodes
}

def qualityVal(Blueprint bp) {
  return numOfGeodes(bp, 1) * bp.number
}

def bps = lines.collect { parseBp it }

def part = 2

if (part == 1) {
  bps = bps.collect { qualityVal it }

  println bps
  println bps.sum()
} else {
  if (!testing) {
    bps = bps[0..2]
  }
  println bps.collect { numOfGeodes(it, 2) }
}


// part 1 1st try 1441 (low)
// part 1 2nd try 1465 (low)
// part 1 3rd try 1466 (20 mins)
// correct nums: [4, 0, 3, 12, 45, 6, 0, 24, 27, 0, 11, 0, 0, 14, 15, 16, 68, 162, 76, 0, 63, 0, 46, 24, 25, 338, 81, 0, 406, 0]

// part 2 1st try 
// [33, 10, 25]
// groovy solution.groovy  1766.38s user 3.83s system 101% cpu 29:07.12 total

