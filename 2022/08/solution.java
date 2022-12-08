import java.util.ArrayList;
import java.util.List;
import java.util.Arrays;
import java.io.BufferedReader;
import java.io.File;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.util.stream.Collectors;

public class Main {
  static int[] parseLine(String input) {
    int[] nums = new int[input.length()];
    for (int i = 0; i < input.length(); ++i) {
      nums[i] = Character.getNumericValue(input.charAt(i)) - Character.getNumericValue('0');
    }
    return nums;
  }

  static int[][] getInput(boolean fromFile) throws FileNotFoundException {
    if (fromFile) {
      var file = new File("input.txt");
      var bufReader = new BufferedReader(new FileReader(file));
      return bufReader.lines().map(str -> parseLine(str)).collect(Collectors.toList()).toArray(new int[0][0]);
    } else {
      var test = """
        30373
        25512
        65332
        33549
        35390""";
      return Arrays.stream(test.split("\n")).map(str -> parseLine(str)).collect(Collectors.toList()).toArray(new int[0][0]);
    }
  }

  static int part1(int[][] forest) {
    int w = forest.length;
    int h = forest[0].length;
    boolean[][] visible = new boolean[w][h];
    for (int i = 0; i < w; ++i) {
      visible[i][0] = true;
      int maks = forest[i][0];
      for (int j = 1; j < h; ++j) {
        if (forest[i][j] > maks) {
          visible[i][j] = true;
          maks = forest[i][j];
        }
      }
      visible[i][h - 1] = true;
      maks = forest[i][h - 1];
      for (int j = h - 2; j >= 0; --j) {
        if (forest[i][j] > maks) {
          visible[i][j] = true;
          maks = forest[i][j];
        }
      }
    }
    for (int i = 0; i < h; ++i) {
      int maks = forest[0][i];
      visible[0][i] = true;
      for (int j = 1; j < w; ++j) {
        if (forest[j][i] > maks) {
          visible[j][i] = true;
          maks = forest[j][i];
        }
      }
      maks = forest[w - 1][i];
      visible[w - 1][i] = true;
      for (int j = w - 2; j >= 0; --j) {
        if (forest[j][i] > maks) {
          visible[j][i] = true;
          maks = forest[j][i];
        }
      }
    }
    int sum = 0;
    for (int i = 0; i < forest.length; ++i) {
      for (int j = 0; j < forest[i].length; ++j) {
        if (visible[i][j]) {
          sum++;
        }
      }
    }
    return sum;
  }

  static int part2(int[][] forest) {
    int best = 0;
    for (int i = 1; i < forest.length - 1; ++i) {
      for (int j = 1; j < forest[i].length - 1; ++j) {
        int score = 1;
        for (int k = i + 1; ; ++k) {
          if (k >= forest.length - 1 || forest[k][j] >= forest[i][j]) {
            score *= k - i;
            break;
          }
        }
        for (int k = i - 1; ; --k) {
          if (k == 0 || forest[k][j] >= forest[i][j]) {
            score *= i - k;
            break;
          }
        }
        for (int k = j + 1;; ++k) {
          if (k == forest[i].length - 1 || forest[i][k] >= forest[i][j]) {
            score *= k - j;
            break;
          }
        }
        for (int k = j - 1;; --k) {
          if (k == 0 || forest[i][k] >= forest[i][j]) {
            score *= j - k;
            break;
          }
        }
        if (score > best) {
          best = score;
        }
      }
    }
    return best;
  }

  static void printInput(int[][] input) {
    for (int i = 0; i < input.length; ++i) {
      for (int j = 0; j < input[i].length; ++j) {
        System.out.print(input[i][j] + " ");
      }
      System.out.println("");
    }
  }

  public static void main(String[] args) throws FileNotFoundException {
    var input = getInput(true);
    System.out.println(part2(input));
  }
}