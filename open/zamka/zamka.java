import java.io.BufferedReader;
import java.io.BufferedOutputStream;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.io.IOException;
import java.util.StringTokenizer;


public class zamka {
    private static int sumOfDigits(int x) {
        String[] c = String.valueOf(x).split("");

        int total = 0;
        for (String d : c) {
            total += Integer.parseInt(d);
        }

        return total;
    }

    public static void main(String[] args) throws Exception {
        int l = scan.nextInt(),
            d = scan.nextInt(),
            x = scan.nextInt(),
            n = d,
            m = 0;

        for (int i = 1; i <= d; i++) {
            if (l <= i && i <= d && sumOfDigits(i) == x && i < n) {
                n = i;
            }

            if (l <= i && i <= d && sumOfDigits(i) == x && i > m) {
                m = i;
            }
        }

        out.println(n);
        out.println(m);
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