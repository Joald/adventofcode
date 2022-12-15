<?php

$test = true;
$part2 = true;
$test_str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

$lines = explode("\n", $test ? $test_str : file_get_contents("input.txt"));
// print($lines[0]);
foreach ($lines as &$line) {
  $line = explode(" -> ", $line);
  foreach ($line as &$pair) {
    $p = explode(",", $pair);
    $pair = [intval($p[0]), intval($p[1])];
  }
}
unset($line);
unset($pair);
$minx = $lines[0][0][0];
$miny = $lines[0][0][1];
$maxx = $minx;
$maxy = $miny;

foreach ($lines as $line) {
  foreach ($line as $pair) {
    $minx = min($minx, $pair[0]);
    $miny = min($miny, $pair[1]);
    $maxx = max($maxx, $pair[0]);
    $maxy = max($maxy, $pair[1]);
  }
}

// for ($j = 0; $j < count($lines); $j++) {
//   for ($i = 0; $i < count($lines[$j]); $i++) {
//     $lines[$j][$i][0] -= $minx;
//     $lines[$j][$i][1] -= $miny;
//     // print("i=" . $i . ", j=" . $j . "\n");
//     // print_r($lines);
//   }
// }
// $maxx -= $minx;
// $maxy -= $miny;

$m = array_fill($minx - 300, $maxx + 301, array_fill(min($miny - 1, 0), $maxy + 10, 0));
// foreach ([$minx, $miny, $maxx, $maxy] as $i) {
//   print($i);
//   print("\n");
// }
// print_r($m);
$insertion = 500;

foreach ($lines as $line) {
  // print("processing {");
  // print_r($line);
  // print("}\n");
  for ($i = 0; $i < count($line) - 1; $i++) {
    $x = $line[$i][0];
    $nx = $line[$i + 1][0];
    $y = $line[$i][1];
    $ny = $line[$i + 1][1];
    $m[$x][$y] = 1;
    while ($x != $nx) {
      $x += $x < $nx ? 1 : -1;
      $m[$x][$y] = 1;
    }
    while ($y != $ny) {
      $y += $y < $ny ? 1 : -1;
      $m[$x][$y] = 1;
    }
  }
}

if ($part2) {
  for ($i = $minx - 300; $i <= $maxx + 300; $i++) {
    $m[$i][$maxy + 2] = 1;
  }
}

function draw($m, $maxx, $maxy) {
  print("draw: (maxx=" . $maxx . ", maxy=" . $maxy . ")\n");

  for ($j = 0; $j <= $maxy; $j++) {
    for ($i = $minx - 10; $i <= $maxx; $i++) {  
      print($m[$i][$j]);
    }
    print("\n");
  }
}

// draw($m, $maxx, $maxy);

$cnt = 0;
while (true) {
  $cnt++;
  $x = $insertion;
  $y = 0;//-$miny;
  // print("cnt=" . $cnt . "\n");
  // if ($cnt < 100) draw($m, $maxx, $maxy);
  while (true) {
    // print("x=" . $x . ", y=" . $y . "\n");
    if ($m[$x][$y + 1] == 0) {
      $y++;
    } else if ($m[$x - 1][$y + 1] == 0) {
      $y++;
      $x--;
    } else if ($m[$x + 1][$y + 1] == 0) {
      $y++;
      $x++;
    } else {
      $m[$x][$y] = 1;
      // print("finish at " . $x . ", " . $y . "\n");
      if ($part2 && $x == $insertion && $y == 0) {
        print($cnt);
        exit();
      }
      break;
    }
    if (!$part2 && ($x < 0 || $x > $maxx || $y > $maxy)) {
      print($cnt - 1);
      exit();
    }

  }
}
?>
