
string[] getInput(bool fromFile) {
  if (fromFile) {
    return File.ReadAllLines("input.txt");
  } else {
    return 
@"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop".Split("\n");
  }
  
}

var lines = /*"noop\naddx 3\naddx -5".Split("\n");*/getInput(true);



void part1() {
int t = 1;
int x = 1;
int nextCycle = 20;
int strengthSum = 0;
foreach (string line in lines) {
  if (line != "noop") {
    int val = Int32.Parse(line.Split(" ")[1]);
    if (t + 1 == nextCycle) {
      Console.WriteLine(String.Format("During the {0}th cycle, register X has the value {1}, so the signal strength is {0} * {1} = {2}.", nextCycle, x, nextCycle * x));
      strengthSum += nextCycle * x;
      nextCycle += 40;
    }
    x += val;
    t += 2;
  } else {
    t++;
  }
  if (t >= nextCycle) {
    Console.WriteLine(String.Format("During the {0}th cycle, register X has the value {1}, so the signal strength is {0} * {1} = {2}.", nextCycle, x, nextCycle * x));
    strengthSum += nextCycle * x;
    nextCycle += 40;
  }
 
}
Console.WriteLine(strengthSum);
}

void part2() {
  int x = 1;
  int t = 1;
  var crt = new Crt();
  foreach (var line in lines) {
//    Console.WriteLine(String.Format("Start cycle {0}: begin executing {1}", crt.sx, line));
    if (line != "noop") {
      crt.processCycle(x);
      crt.processCycle(x);
      x += Int32.Parse(line.Split(" ")[1]);
    } else {
      crt.processCycle(x);
    }
  }
  Console.WriteLine(crt.getScreen());
}

part2();


public class Crt {
  public int sx = 1;
  string line = "";
  string[] lines = new string[6];
  int curLine = 0;
  public void processCycle(int spr_x) {
//    Console.WriteLine(String.Format("During cycle {0}: CRT draws pixel in position {1}, sprite is at {2}, therefore we draw {3}", sx + curLine * 40, sx - 1, spr_x, Math.Abs(spr_x - (sx - 1)) < 2 ? '#' : '.'));
    if (Math.Abs(spr_x - (sx - 1)) < 2) {
      line += '#';
    } else {
      line += '.';
    }
    sx++;
    if (sx == 41) {
      lines[curLine] = line;
      line = "";
      sx = 1;
      curLine++;
    }
  }
  public string getScreen() {
    return String.Join('\n', lines);
  }
}

