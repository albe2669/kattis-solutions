import java.io.BufferedReader;
import java.io.BufferedOutputStream;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.io.IOException;
import java.util.HashMap;
import java.util.StringTokenizer;

public class pokerhand {
    public static void main(String[] args) throws Exception {
        HashMap<Character, Integer> cards = new HashMap<Character, Integer>();
        for (int i = 0; i < 5; i++) {
            char c = scan.next().toCharArray()[0];
            if (cards.get(c) != null) {
                cards.put(c, cards.get(c) + 1);
            } else {
                cards.put(c, 1);
            }
        }

        int count = 0;
        for (Integer i : cards.values()) {
            if (i > count) {
                count = i;
            }
        }
        out.println(count);
        out.close();
    }

    // -----------PrintWriter for faster output---------------------------------
    public static PrintWriter out = new PrintWriter(new BufferedOutputStream(System.out));

    public static MyScanner scan = new MyScanner();

    // -----------MyScanner class for faster input----------
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