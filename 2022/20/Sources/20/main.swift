import Foundation

class Permutation: CustomStringConvertible {
  var perm: [Int]
  var inv: [Int]

  init(size: Int) {
    perm = [Int](0..<size)
    inv = [Int](0..<size)
  }

  func swapImage(_ i1: Int, _ i2: Int) {
    let pi1 = inv[i1]
    let pi2 = inv[i2]
    perm.swapAt(pi1, pi2)
    inv.swapAt(i1, i2)
  }

  var description: String {
    return "perm: \(perm), inv: \(inv)"
  }
}


func part1(input: String) {
  let lines = input.split(separator: "\n")
  var nums = lines.map { Int($0)! }
  // print(nums.count)
  let origs = nums
  // var perm: [Int] = []
  // for i in 0..<nums.count {
  //   perm.append(i)
  // }
  let perm = Permutation(size: nums.count)
  for i in 0..<nums.count {
    // print(nums)
    // let bads = spots.filter { $0.value < 0 }
    let num = origs[i]
    var pos = perm.perm[i]
    let absnum = abs(num)
    let dir: Int = absnum == 0 ? 0 : num / absnum
    let delta = absnum % (nums.count - 1)
    // if i == 4 {
    //   print(i, num, pos, absnum, dir, delta)
    //   print(bads)
    //   print(nums)
    //   return
    // }
    if delta == 0 {
      continue
    }
    for _ in 0..<delta {
      if (pos == 0 && dir == -1) {
        // for j in 1..<nums.count {
        //   spots[nums[j]]! -= 1
        //   nums.swapAt(j, j - 1)
        // }
        // spots[nums[nums.count - 1]] = 0
        nums.swapAt(0, nums.count - 1)
        perm.swapImage(0, nums.count - 1)
        pos = nums.count - 1


      } else if (pos == nums.count - 1 && dir == 1) {
        // for j in stride(from: nums.count - 2, through: 0, by: -1) {
        //   spots[nums[j]]! += 1
        //   nums.swapAt(j, j + 1)
        // }
        // spots[nums[0]] = nums.count - 1
        nums.swapAt(0, nums.count - 1)
        perm.swapImage(0, nums.count - 1)
        pos = 0
      } else {
        // spots[nums[pos + dir]] = pos
        nums.swapAt(pos, pos + dir)
        perm.swapImage(pos, pos + dir)
        pos += dir
        // if pos < 0 {
        //   print(i)
        // }
      }
    }
    // spots[num] = pos
  }
  // print(nums)

  var zerospot = -1
  for (i, orig) in origs.enumerated() {
    if orig == 0 {
      zerospot = perm.perm[i]
      break
    }
  }
  print(nums[(zerospot + 1000) % nums.count], nums[(zerospot + 2000) % nums.count], nums[(zerospot + 3000) % nums.count])
}


func part2(input: String) {
  let lines = input.split(separator: "\n")
  var nums = lines.map { Int($0)! * 811589153 }
  let origs = nums
  let perm = Permutation(size: nums.count)
  for _ in 0..<10 {
    for i in 0..<nums.count {
      let num = origs[i]
      var pos = perm.perm[i]
      let absnum = abs(num)
      let dir: Int = absnum == 0 ? 0 : num / absnum
      let delta = absnum % (nums.count - 1)
      if delta == 0 {
        continue
      }
      for _ in 0..<delta {
        if (pos == 0 && dir == -1) {
          nums.swapAt(0, nums.count - 1)
          perm.swapImage(0, nums.count - 1)
          pos = nums.count - 1
        } else if (pos == nums.count - 1 && dir == 1) {
          nums.swapAt(0, nums.count - 1)
          perm.swapImage(0, nums.count - 1)
          pos = 0
        } else {
          nums.swapAt(pos, pos + dir)
          perm.swapImage(pos, pos + dir)
          pos += dir
        }
      }
    }
  }
  var zerospot = -1
  for (i, orig) in origs.enumerated() {
    if orig == 0 {
      zerospot = perm.perm[i]
      break
    }
  }
  let res = [nums[(zerospot + 1000) % nums.count], nums[(zerospot + 2000) % nums.count], nums[(zerospot + 3000) % nums.count]]
  print(res)
  print(res.reduce(0, { $0 + $1 }))
}



var inputs: [String] = []
/*
4 -3 2
-18 -6 6
0 0 0
*/
inputs.append("""
1
2
-3
3
-2
0
4
""")
inputs.append("""
1
0
6
-6
-20
-21
-18
""")
inputs.append("""
8
2
32
-41
6
29
-4
6
-8
8
-3
-8
3
-5
0
-1
2
1
10
-9
""")


let file = "input.txt"
if let dir = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first {
  let fileURL = dir.appendingPathComponent(file) 
  let text = try! String(contentsOf: fileURL)
  inputs.append(text)
}

for input in inputs {
  part2(input: input)
}

// part 1 1st try: 12187 (too high)
// part 1 2nd try: -3686 (no direction)
// part 1 3rd try: 10547
// part 1 4th try: -6677
// part 1 5th try: 10707 (finally figured out the input lines aren't unique...)