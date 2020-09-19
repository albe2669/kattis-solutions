import java.io.BufferedReader;
import java.io.BufferedOutputStream;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.io.IOException;
import java.util.StringTokenizer;


public class trik {
    public static void main(String[] args) throws Exception {
        char[] moves = scan.next().toCharArray();
        boolean[] cups = new boolean[]{true, false, false};
        boolean tmp;

        for (char c : moves) {
            if (c == 'A') {
                tmp = cups[1];
                cups[1] = cups[0];
                cups[0] = tmp;
            } else if (c == 'B') {
                tmp = cups[1];
                cups[1] = cups[2];
                cups[2] = tmp;
            } else {
                tmp = cups[0];
                cups[0] = cups[2];
                cups[2] = tmp;
            }
        }

        for (int i = 0; i < cups.length; i++) {
            if(cups[i]) {
                out.println(i+1);
            }
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