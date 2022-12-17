import 'dart:math';

class Coord {
  final int x;
  final int y;
  const Coord(this.x, this.y);
  Coord plusX(int delta) => Coord(x + delta, y);
  Coord plusY(int delta) => Coord(x, y + delta);
}

class CoordMut {
  int x;
  int y;
  CoordMut(this.x, this.y);

  Coord get imm => Coord(x, y);
}

enum Rock {
  // (0,0) is bottom left with ys increasing upwards
  hLine([Coord(0, 0), Coord(1, 0), Coord(2, 0), Coord(3, 0)]),
  plus([Coord(0, 1), Coord(1, 0), Coord(1, 1), Coord(2, 1), Coord(1, 2)]),
  el([Coord(0, 0), Coord(1, 0), Coord(2, 0), Coord(2, 1), Coord(2, 2)]),
  vLine([Coord(0, 0), Coord(0, 1), Coord(0, 2), Coord(0, 3)]),
  square([Coord(0, 0), Coord(1, 0), Coord(0, 1), Coord(1, 1)]);

  const Rock(this.shape);

  final List<Coord> shape;
  List<Coord> movedTo(Coord other) =>
      [for (var coord in shape) Coord(coord.x + other.x, coord.y + other.y)];
  int get maxY => shape.fold(0, (acc, coord) => max(acc, coord.y));
}

const rocks = [Rock.hLine, Rock.plus, Rock.el, Rock.vLine, Rock.square];

bool anyCollide(List<Coord> coords, List<List<bool>> board) =>
    coords.any((coord) => board[coord.x][coord.y]);

void printBoard(List<List<bool>> board) {
  for (int y = 30; y >= 0; y--) {
    var line = '';
    for (int x = 0; x < board.length; x++) {
      final wall = x == 0 || x == board.length - 1;
      line += y == 0 && wall
          ? '+'
          : y == 0
              ? '-'
              : wall
                  ? '|'
                  : board[x][y]
                      ? '#'
                      : '.';
    }
    print(line);
  }
}

List<int> dropRock(int maxY, List<List<bool>> board, Rock rock,
    String jetStream, int nextJet, bool toPrint) {
  var curPos = Coord(3, maxY + 4);
  while (true) {
    var newPos = jetStream[nextJet] == '<' ? curPos.plusX(-1) : curPos.plusX(1);
    if (toPrint) print("next is ${jetStream[nextJet]}");
    nextJet = (nextJet + 1) % jetStream.length;
    if (!anyCollide(rock.movedTo(newPos), board)) {
      curPos = newPos;
    }
    newPos = curPos.plusY(-1);
    if (anyCollide(rock.movedTo(newPos), board)) {
      break;
    }
    curPos = newPos;
  }
  for (var coord in rock.movedTo(curPos)) {
    board[coord.x][coord.y] = true;
  }

  return [max(maxY, curPos.y + rock.maxY), nextJet];
}

int part1(String jetPattern) {
  var board = [
    for (var x = -1; x <= 7; x++)
      [for (var y = -1; y < 9001; y++) x == -1 || y == -1 || x == 7]
  ];
  var maxY = 0;
  var nextJet = 0;
  for (var dropped = 0; dropped < 2022; ++dropped) {
    final toPrint = false;
    if (toPrint) {
      print("dropped $dropped");
      printBoard(board);
    }
    final rock = rocks[dropped % rocks.length];
    final res = dropRock(maxY, board, rock, jetPattern, nextJet, false);
    maxY = res[0];
    nextJet = res[1];
  }

  return maxY;
}

bool getBoard(List<List<bool>> board, int x, int y, int floorY) =>
    board[x][y + floorY];

int topInaccessibleY(List<List<bool>> board, int maxY, int floorY) {
  var prev = [for (var x = 0; x < 7; ++x) !getBoard(board, x, maxY, floorY)];
  var curY = maxY - 1;
  while (curY > floorY) {
    var cur = [
      for (var x = 0; x < 7; ++x) prev[x] && !getBoard(board, x, curY, floorY)
    ];
    for (var x = 0; x < 7; ++x) {
      if (!cur[x]) continue;
      var xl = x - 1;
      while (xl >= 0 && !cur[xl] && !getBoard(board, xl, curY, floorY)) {
        cur[xl] = true;
        xl--;
      }
      var xr = x + 1;
      while (xr < 7 && !cur[xr] && !getBoard(board, xr, curY, floorY)) {
        cur[xr] = true;
        xr++;
      }
    }
    if (cur.every((element) => element == false)) break;

    prev = cur;
    curY++;
  }
  return curY;
}

int part2(String jetPattern) {
  final board = [
    for (var x = -1; x <= 7; x++)
      [for (var y = -1; y < 10000; y++) x == -1 || y == -1 || x == 7]
  ];
  var maxY = 0;
  var nextJet = 0;
  final memory = {
    -1: {
      -1: [-1]
    }
  };
  var toAddToResult = 0;
  var found = false;
  const target = 1000000000000;
  for (var dropped = 0; dropped < target; ++dropped) {
    final toPrint = false;
    if (toPrint) {
      print("dropped $dropped");
      printBoard(board);
    }
    final rockPos = dropped % rocks.length;
    final rock = rocks[rockPos];
    final res = dropRock(maxY, board, rock, jetPattern, nextJet, false);
    maxY = res[0];
    nextJet = res[1];
    if (!found) {
      final memVal = memory[rockPos]?[nextJet];
      if (memVal == null) {
        if (memory[rockPos] == null) memory[rockPos] = {};
        memory[rockPos]?[nextJet] = [maxY, dropped];
      } else if (dropped > 2022) {
        print("Found!");
        found = true;
        final lastH = memVal[0];
        final lastDropped = memVal[1];
        final cycH = maxY - lastH;
        final cycLen = dropped - lastDropped;

        final left = target - dropped;
        final cycles = left ~/ cycLen;
        dropped += cycles * cycLen;
        toAddToResult = cycH * cycles;
        memory.clear();
      }
    }
    print(memory.values.fold(0, (acc, mp) => (acc as int) + mp.keys.length));
  }
  // test exampl 1514285714288
  // fst try was 1531594202892 (too high)
  // snd try was 1531594202890
  // thd try was 1531594200477 (too high)
  // fth try was 1523615160362 (after letting it run for 2022 before predicting the rest)

  return maxY + toAddToResult;
}
