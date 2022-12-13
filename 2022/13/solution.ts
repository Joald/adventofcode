import * as fs from 'fs';
const util = require('util');

const inspect = (obj: any) => util.inspect(obj, {
  showHiddem: false,
  depth: null,
  colors: true,
  maxArrayLength: null,
});

interface nested<T> extends Array<T | nested<T>> { }

function isInOrder(left: nested<number>, right: nested<number>): number {
  if (left.length == 0 && right.length == 0) {
    return 0;
  }
  if (left.length == 0) {
    return -1;
  }
  if (right.length == 0) {
    return 1;
  }

  if (typeof left[0] === "number" && typeof right[0] === "number") {
    return left[0] === right[0] ? isInOrder(left.slice(1), right.slice(1)) : left[0] - right[0];
  } else if (typeof left[0] !== "number" && typeof right[0] !== "number") {
    const res = isInOrder(left[0], right[0]);
    return res === 0 ? isInOrder(left.slice(1), right.slice(1)) : res;
  } else {
    const [rlist, llist] = typeof left[0] === "number" ? [right[0] as nested<number>[], [left[0]]] : [[right[0]], left[0]];
    const res = isInOrder(llist, rlist);
    return res === 0 ? isInOrder(left.slice(1), right.slice(1)) : res;
  }
}

function part12(input: string[], partno: number): number {
  const ps: [nested<number>, nested<number>][] = [];
  for (let i = 0; i < input.length; i += 3) {
    ps.push([eval(input[i]), eval(input[i + 1])]);
  }

  if (partno == 1) {
    let indexsum = 0;
    ps.forEach(([left, right], i) => {
      if (isInOrder(left, right) < 0) {
        console.log("in order:", inspect(left), inspect(right), "at", i)
        indexsum += i + 1;
      }
    })
    return indexsum
  } else {
    const tosort = ps.reduce((acc, [l, r]) => { acc.push(l, r); return acc; }, [] as nested<number>[]);
    const div1 = [[2]], div2 = [[6]];
    tosort.push(div1, div2);
    tosort.sort(isInOrder);
    console.log("in order:", inspect(tosort));
    return (tosort.indexOf(div1) + 1) * (tosort.indexOf(div2) + 1);
  }
}

const example = `[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]`;

fs.readFile('input.txt', 'utf8', (err, data) => {
  if (err) {
    console.error(err);
    return;
  }
  console.log(part12(example.split("\n"), 2))
  console.log(part12(data.split("\n"), 2));
});

/*
22116 too high
21922
*/ 