const fs = require('fs');

class Queue {
  constructor() {
    this.q = []
  }

  push = elem => this.q.push(elem);

  pop = () => {
    const rv = this.q[0];
    this.q = this.q.slice(1);
    return rv
  }

  empty = () => this.q.length == 0
}

function part12(input, part) {
  lines = input.split("\n")
  spos = [-1, -1]
  epos = [-1, -1]
  
  for (const [i, elem] of lines.entries()) {
    if (spos[1] == -1) {
      spos = [i, elem.indexOf('S')];
    }
    if (epos[1] == -1) {
      epos = [i, elem.indexOf('E')];
    }
  }
  console.log(`spos=${spos}, epos=${epos}`);
  lines[spos[0]] = lines[spos[0]].replace('S', 'a');
  lines[epos[0]] = lines[epos[0]].replace('E', 'z');

  q = new Queue();
  if (part == 1) {
    q.push([spos, 0]);
  } else {
    for (const [i, elem] of lines.entries()) {
      for (let j = 0; j < elem.length; ++j) {
        if (elem[j] == 'a') {
          q.push([[i, j], 0]);
        }
      }
    }
  }


  const vis = new Array(lines.length).fill().map(_ => new Array(lines[0].length).fill(false));
  vis[spos[0]][spos[1]] = true;
  while (!q.empty()) {
    const [[x, y], len] = q.pop();
    if (x == epos[0] && y == epos[1]) {
      return len;
    }
    [[0, -1], [0, 1], [-1, 0], [1, 0]].forEach(([xdelta, ydelta]) => {
      const [xi, yi] = [x + xdelta, y + ydelta];
      if (xi >= 0 && xi < lines.length && yi >= 0 && yi < lines[xi].length && !vis[xi][yi] && lines[xi].charCodeAt(yi) <= lines[x].charCodeAt(y) + 1) {
        q.push([[xi, yi], len + 1]);
        vis[xi][yi] = true;
      }
    });
  }
  console.log(vis.map(row => row.map(b => b ? '1' : '0').join('')))
  return "not found"
}

fs.readFile('input.txt', 'utf8', (err, data) => {
  if (err) {
    console.error(err);
    return;
  }
  console.log(part12(data, 2))
});
