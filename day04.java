import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;

public class day04 {
  public record C(int i, int j) {
    public C move(C dir, int dist) {
      return new C(this.i + dir.i * dist, this.j + dir.j * dist);
    }
    public C rot() {
      return new C(-this.j, this.i);
    }
  }

  public static void main(String[] args) throws IOException {
    Map<Character, Set<C>> indexMap = new HashMap<>();
    indexMap.put('X', new HashSet<>());
    indexMap.put('M', new HashSet<>());
    indexMap.put('A', new HashSet<>());
    indexMap.put('S', new HashSet<>());
    try (BufferedReader br = new BufferedReader(new FileReader(args[0]))) {
      String line;
      int i = 0;
      while ((line = br.readLine()) != null) {
        char[] chr = line.toCharArray();
        for (int j = 0; j < chr.length; j++) {
          var map = indexMap.get(chr[j]);
          if (map == null) {
            continue;
          }
          map.add(new C(i, j));
        }
        i++;
      }
    }
    List<C> xdirs = Arrays.asList(new C(-1, 1), new C(1, 1), new C(1, -1), new C(-1, -1));
    List<C> vdirs = Arrays.asList(new C(-1, 0), new C(1, 0), new C(0, 1), new C(0, -1));
    List<C> dirs = new ArrayList<>();
    dirs.addAll(xdirs);
    dirs.addAll(vdirs);
    int cnt = 0;

    for (C dir : dirs) {
      cnt += indexMap.get('X').stream().filter(idx -> 
          indexMap.get('M').contains(idx.move(dir, 1)) &&
          indexMap.get('A').contains(idx.move(dir, 2)) &&
          indexMap.get('S').contains(idx.move(dir, 3))
      ).count();
    }
    System.out.println("part1: " + cnt);

    cnt = 0;
    for (C dir : xdirs) {
      cnt += indexMap.get('A').stream().filter(idx -> 
          indexMap.get('M').contains(idx.move(dir, 1)) &&
          indexMap.get('S').contains(idx.move(dir, -1)) &&
          (indexMap.get('M').contains(idx.move(dir.rot(), 1)) &&
          indexMap.get('S').contains(idx.move(dir.rot(), -1)) || 
          indexMap.get('M').contains(idx.move(dir.rot(), -1)) &&
          indexMap.get('S').contains(idx.move(dir.rot(), 1)))
      ).count();
    }
    System.out.println("part2: " + cnt/2);
  }
}