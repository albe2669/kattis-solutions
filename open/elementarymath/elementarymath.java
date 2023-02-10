import java.io.BufferedReader;
import java.io.BufferedOutputStream;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.io.IOException;
import java.util.StringTokenizer;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class elementarymath {
    public static int addValue(long value, Map<Long, Integer> uniqueAnswers, Map<Integer, Long> calculatedValues) {
        if (uniqueAnswers.get(value) == null) {
            uniqueAnswers.put(value, uniqueAnswers.size());
            calculatedValues.put(uniqueAnswers.get(value), value);
        }
        return uniqueAnswers.get(value);
    }

    public static boolean connectValue(boolean[] visited, int current) {
        if (current == -1) {
            return true;
        }

        for (int position : adjacencyList.get(current)) {
            if (!visited[position]) {
                visited[position] = true;
                if (connectValue(visited, positions[position])) {
                    positions[position] = current;
                    return true;
                }
            }
        }
        return false;
    }

    public static class Pair {

        long a;
        long b;

        Pair(long a, long b) {
            this.a = a;
            this.b = b;
        }
    }

    public static List<List<Integer>> adjacencyList;
    public static int[] positions;

    public static void main(String[] args) throws Exception {
        int numPairs = Integer.parseInt(scan.nextLine());
        Map<Long, Integer> uniqueAnswers = new HashMap<>();
        Map<Integer, Long> calculatedValues = new HashMap<>();

        Pair[] pairs = new Pair[numPairs];
        positions = new int[numPairs * 3];
        Arrays.fill(positions, -1);

        adjacencyList = new ArrayList<>();
        for (int i = 0; i < numPairs; i++) {
            adjacencyList.add(new ArrayList<>(3));
        }

        for (int i = 0; i < numPairs; i++) {
            String[] data = scan.nextLine().split(" ");
            long a = Integer.parseInt(data[0]);
            long b = Integer.parseInt(data[1]);
            pairs[i] = new Pair(a, b);
            adjacencyList.get(i).add(addValue(a * b, uniqueAnswers, calculatedValues));
            adjacencyList.get(i).add(addValue(a + b, uniqueAnswers, calculatedValues));
            adjacencyList.get(i).add(addValue(a - b, uniqueAnswers, calculatedValues));
        }

        int numConnected = 0;
        for (int i = 0; i < numPairs; i++) {
            boolean[] visited = new boolean[numPairs * 3];
            numConnected = (connectValue(visited, i)) ? numConnected + 1 : 0;
        }

        if (numConnected < numPairs) {
            out.println("impossible");
            return;
        }

        long[] answers = new long[numPairs];
        for (int i = 0; i < uniqueAnswers.size(); i++) {
            if (positions[i] != -1) {
                answers[positions[i]] = calculatedValues.get(i);
            }
        }

        for (int i = 0; i < numPairs; i++) {
            long a = pairs[i].a;
            long b = pairs[i].b;
            String operator = " * ";

            if (a + b == answers[i]) {
                operator = " + ";
            } else if (a - b == answers[i]) {
                operator = " - ";
            }
            out.println(a + operator + b + " = " + answers[i]);
        }

        out.close();
    }

    //-----------PrintWriter for faster output---------------------------------
    public static PrintWriter out = new PrintWriter(new BufferedOutputStream(System.out));
    
    public static MyScanner scan = new MyScanner();

    //-----------MyScanner class for faster input----------
    public static class MyScanner {
        BufferedReader br;
        StringTokenizer st;

        public MyScanner() {
            br = new BufferedReader(new InputStreamReader(System.in));
        }

        String next() {
            while (st == null || !st.hasMoreElements()) {
                try {
                    st = new StringTokenizer(br.readLine());
                } catch (IOException e) {
                    e.printStackTrace();
                }
            }
            return st.nextToken();
        }

        int nextInt() {
            return Integer.parseInt(next());
        }

        long nextLong() {
            return Long.parseLong(next());
        }

        double nextDouble() {
            return Double.parseDouble(next());
        }

        String nextLine() {
            String str = "";
            try {
                str = br.readLine();
            } catch (IOException e) {
                e.printStackTrace();
            }
            return str;
        }

    }
}