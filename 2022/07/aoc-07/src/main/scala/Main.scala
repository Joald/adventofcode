import scala.io.Source

sealed trait FsNode

case class File(name: String, size: BigInt) extends FsNode
case class Dir(name: String, files: List[FsNode]) extends FsNode

sealed trait Cmd 
case class Ls() extends Cmd
case class Cd(dir: String) extends Cmd

def parseCmd(line: String): Cmd =
  if line startsWith "$ cd" then Cd(line.split(" ")(2)) else Ls()

def parseFile(line: String): FsNode = 
  val spl = line.split(" ");
  if spl(0) == "dir" then Dir(spl(1), Nil) else File(spl(1), BigInt(spl(0)))

def parseDir(source: List[String]): (List[FsNode], List[String]) = source match 
  case h :: t if !h.startsWith("$ ") => {
    val (nodes, rest) = parseDir(t);
    (parseFile(h)::nodes, rest)
  }
  case _ => (Nil, source)

def replaceSubdir[T](dir: Dir, subdir: String, replacer: (Dir => (FsNode, List[T]))): (Dir, List[T]) = dir.files match {
  case (d@Dir(name, _)) :: t if name == subdir => {
    val (subnode, rest) = replacer(d); 
    // println("replaceSubdir found:\n  " + name + "")
    (Dir(dir.name, subnode :: t), rest)
  }
  case h :: t => {
    val (subnode, rest) = replaceSubdir(Dir(dir.name, t), subdir, replacer); 
    (Dir(dir.name,h :: subnode.files), rest)
  }
  case Nil => println("ERROR: replaceSubdir(" + dir + ", " + subdir + ")"); (dir, Nil)
}

def parse(source: List[String], dir: Dir): (FsNode, List[String]) = source match {
  case h :: t if h startsWith "$ " => parseCmd(h) match {
    case Ls() => {
      val (files, rest) = parseDir(t);
      // print("\n$ ls: ");print(files);print(" at ");println(dir);
      // println("@ls: parse(" + rest + ", " + Dir(dir.name, files) + ")\n")
      parse(rest, Dir(dir.name, files))
    }
    case Cd(dirName) => {
      // print("\n$ cd "); print(dirName); print(" at ");println(dir);
      if dirName == ".." then (dir, t) else if dirName == "/" then parse(t, dir) else {
        val (newDir, rest) = replaceSubdir(dir, dirName, (parse _).curried(t));
        // println("\nTRANSFORM: \n  " + dir + "\n  -> \n  " + newDir  + "\n  at cd " + dirName);
        parse(rest, newDir)
      }
    }
  }
  case _ => 
    // println("CALLED:\n  parse(" + source + ", " + dir + ")\n  ->\n  " + (dir, Nil) + "\n"); 
    (dir, Nil)
}

// def traverse[T](t: FsNode => List[T], pred: List[T] => List[T])(node: FsNode): List[T] = node match {
//   case f@File(_) => t(f)
//   case d@Dir(_, fs) => comb(t(d) ::: fs.concatMap(traverse(t)))
// }

// def getSz(node: FsNode): (BigInt, BigInt) = node match {// (direct_size, acc_size)
//   case File(_, sz) => (sz, 
//   case Dir(_, _) => 0
// }
def part1(node: FsNode): (BigInt, BigInt) = node match {
  case File(_, sz) => (sz, 0)
  case Dir(_, files) => {
    val (szs, accs) = files.map(part1).unzip
    val sz = szs.foldLeft(BigInt(0))(_ + _);
    val acc = accs.foldLeft(BigInt(0))(_ + _);
    (sz, if sz <= 100000 then sz + acc else acc)
  }
}

def part2(node: Dir): BigInt = {
  val (total, _) = part1(node);
  val minDel = total - 40000000;
  def helper(node: FsNode):(BigInt, BigInt) = node match {
    case File(_, sz) => (sz, BigInt(70000000))
    case Dir(_, files) => {
      val (szs, accs) = files.map(helper).unzip
      val sz = szs.foldLeft(BigInt(0))(_ + _);
      val acc = accs.foldLeft(BigInt(70000000))(_ min _);
      (sz, if sz >= minDel && sz < acc then sz else acc)
    }
  }
  helper(node)(1)
}

def printLevels(dir: Dir, level: Int): Unit = {
  println("|" * level + dir.name);
  val lvl = level + 1;
  for (f <- dir.files) {
    f match {
      case File(name, size) => println("|" * lvl + name + ": " + size)
      case d@Dir(_, _) => printLevels(d, lvl)
    }
  }
}

@main def main: Unit = 
  val lines = Source.fromFile("input.txt").getLines.toList
//   val lines = """$ cd /
// $ ls
// dir a
// 14848514 b.txt
// 8504156 c.dat
// dir d
// $ cd a
// $ ls
// dir e
// 29116 f
// 2557 g
// 62596 h.lst
// $ cd e
// $ ls
// 584 i
// $ cd ..
// $ cd ..
// $ cd d
// $ ls
// 4060174 j
// 8033020 d.log
// 5626152 d.ext
// 7214296 k""".split("\n").toList
  val (dir, _) = parse(lines, Dir("/", Nil));
  dir match { case d@Dir(_, _) => printLevels(d, 0); println(part2(d)) };
  // println(part1(dir));
