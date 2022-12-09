
import java.io.*
import kotlin.collections.*
import kotlin.math.*

enum class Direction {
  RIGHT, LEFT, UP, DOWN
}
data class Command(val dir: Direction, val amount: Int)

fun parseLine(line: String): Command {
  val dir = when (line.get(0)) {
    'R' -> Direction.RIGHT
    'L' -> Direction.LEFT
    'U' -> Direction.UP
    'D' -> Direction.DOWN
    else -> throw Exception("first char of line is " + line.get(0))
  };
  return Command(dir, line.split(' ').get(1).toInt())
}

fun getInput(fromFile: Boolean): List<Command> {
  val small = """
    |R 4
    |U 4
    |L 3
    |D 1
    |R 4
    |D 1
    |L 5
    |R 2
    """;
  val large = """
    |R 5
    |U 8
    |L 8
    |D 3
    |R 17
    |D 10
    |L 25
    |U 20
    """;
  val lines = 
    if (fromFile) 
      File("input.txt").useLines { it.toList() }
    else 
      large.trimMargin().split("\n");
  return lines.map { parseLine(it) }
}

var IntArray.first
  set(value) = set(0, value)
  get() = get(0);
var IntArray.second
  set(value) = set(1, value)
  get() = get(1);

fun printPos(pos: Array<IntArray>, bound: Int) {
  val poset = HashMap<Pair<Int, Int>, Int>();
  for ((i, p: IntArray) in pos.withIndex()) {
    val pair = Pair<Int, Int>(p.first, p.second);
    if (!poset.contains(pair)) {
      poset[pair] = i;
    } else {
      println("${poset[pair]} covers $i");
    }
  }
  for (j in -bound..bound) {
    for (i in -bound..bound) {
      if (poset.contains(Pair(i, j)))
        print(poset[Pair(i, j)])
      else 
        print('.');
    }
    println();
  }
  println();
}

fun part12(cmds: List<Command>, knotCount: Int): Int {
  val visited = HashSet<Pair<Int, Int>>();
  visited.add(Pair(0, 0));
  val pos = Array(knotCount) { intArrayOf(0, 0) };
  for ((dir, amount) in cmds) {
    for (_i in 1..amount) {
      when (dir) {
        Direction.UP -> pos[0].second--
        Direction.DOWN -> pos[0].second++
        Direction.LEFT -> pos[0].first--
        Direction.RIGHT -> pos[0].first++
      };
      for (i in 1..knotCount - 1) {
        val hdist: Int = abs(pos[i - 1].first - pos[i].first);
        val vdist: Int = abs(pos[i - 1].second - pos[i].second);
        if (hdist + vdist == 2) {
          if (pos[i - 1].first == pos[i].first + 2) {
            pos[i].first++;
          } else if (pos[i - 1].first == pos[i].first - 2) {
            pos[i].first--;
          } else if (pos[i - 1].second == pos[i].second + 2) {
            pos[i].second++;
          } else if (pos[i - 1].second == pos[i].second - 2) {
            pos[i].second--;
          }
        } else if (hdist == 2 && vdist == 2) {
          for (j in 0..1) {
            pos[i][j] += pos[i - 1][j];
            pos[i][j] /= 2;
          }
        } else if (hdist + vdist > 2) {
          if (pos[i - 1].first == pos[i].first + 2) {
            pos[i].first++;
            pos[i].second = pos[i - 1].second;
          } else if (pos[i - 1].first == pos[i].first - 2) {
            pos[i].first--;
            pos[i].second = pos[i - 1].second;
          } else if (pos[i - 1].second == pos[i].second + 2) {
            pos[i].second++;
            pos[i].first = pos[i - 1].first;
          } else if (pos[i - 1].second == pos[i].second - 2) {
            pos[i].second--;
            pos[i].first = pos[i - 1].first;
          }
        }
      }
      visited.add(Pair(pos[knotCount - 1].first, pos[knotCount - 1].second));
    }
  }
  return visited.size;
}

fun main(args: Array<String>) {
  val input = getInput(true);
  println(part12(input, 10));
}
