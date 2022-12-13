"use strict";
exports.__esModule = true;
var fs = require("fs");
var util = require('util');
var inspect = function (obj) { return util.inspect(obj, {
    showHiddem: false,
    depth: null,
    colors: true,
    maxArrayLength: null
}); };
function isInOrder(left, right) {
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
    }
    else if (typeof left[0] !== "number" && typeof right[0] !== "number") {
        var res = isInOrder(left[0], right[0]);
        return res === 0 ? isInOrder(left.slice(1), right.slice(1)) : res;
    }
    else {
        var _a = typeof left[0] === "number" ? [right[0], [left[0]]] : [[right[0]], left[0]], rlist = _a[0], llist = _a[1];
        var res = isInOrder(llist, rlist);
        return res === 0 ? isInOrder(left.slice(1), right.slice(1)) : res;
    }
}
function part12(input, partno) {
    var ps = [];
    for (var i = 0; i < input.length; i += 3) {
        ps.push([eval(input[i]), eval(input[i + 1])]);
    }
    if (partno == 1) {
        var indexsum_1 = 0;
        ps.forEach(function (_a, i) {
            var left = _a[0], right = _a[1];
            if (isInOrder(left, right) < 0) {
                console.log("in order:", inspect(left), inspect(right), "at", i);
                indexsum_1 += i + 1;
            }
        });
        return indexsum_1;
    }
    else {
        var tosort = ps.reduce(function (acc, _a) {
            var l = _a[0], r = _a[1];
            acc.push(l, r);
            return acc;
        }, []);
        var div1 = [[2]], div2 = [[6]];
        tosort.push(div1, div2);
        tosort.sort(isInOrder);
        console.log("in order:", inspect(tosort));
        return (tosort.indexOf(div1) + 1) * (tosort.indexOf(div2) + 1);
    }
}
var example = "[1,1,3,1,1]\n[1,1,5,1,1]\n\n[[1],[2,3,4]]\n[[1],4]\n\n[9]\n[[8,7,6]]\n\n[[4,4],4,4]\n[[4,4],4,4,4]\n\n[7,7,7,7]\n[7,7,7]\n\n[]\n[3]\n\n[[[]]]\n[[]]\n\n[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]";
fs.readFile('input.txt', 'utf8', function (err, data) {
    if (err) {
        console.error(err);
        return;
    }
    // console.log(part12(example.split("\n"), 2))
    console.log(part12(data.split("\n"), 2));
});
/*
22116 too high
*/ 
