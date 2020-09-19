import java.io.BufferedReader;
import java.io.BufferedOutputStream;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.io.IOException;
import java.util.StringTokenizer;

import java.lang.Math;

public class transitwoes {
    public static void main(String[] args) throws Exception {
        int s = scan.nextInt(),
            t = scan.nextInt(),
            n = scan.nextInt();

        int total = 0;
        int[] d = new int[n+1];
        int[] b = new int[n+1];
        int[] c = new int[n+1];
        
        for (int i = 0; i <= n; i++) {
            d[i] = scan.nextInt();
        }
        
        for (int i = 0; i < n; i++) {
            b[i] = scan.nextInt();
        }

        for (int i = 0; i < n; i++) {
            c[i] = scan.nextInt();
        }

        for (int i = 0; i < n; i++) {
            total += d[i];
            
            total += Math.abs(total % c[i]) + b[i];
        }

        
        if (total + d[n] < t) {
            out.println("yes");
        } else {
            out.println("no");
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