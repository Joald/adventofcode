import 'package:aoc17/aoc17.dart' as aoc17;
import 'dart:io';

void main(List<String> arguments) async {
  const testing = false;
  final str = testing
      ? '>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>'
      : (await File('input.txt').readAsString()).replaceAll('\n', '');

  print('Result: ${aoc17.part2(str)}!');
}
