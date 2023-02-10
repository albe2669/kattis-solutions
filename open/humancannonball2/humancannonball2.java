import java.lang.Math;

import java.io.BufferedReader;
import java.io.BufferedOutputStream;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.io.IOException;
import java.util.StringTokenizer;


public class humancannonball2 {
    public static void main(String[] args) throws Exception {
        int n = scan.nextInt();
        for (int i = 0; i < n; i++) {
            
            double  v0      = (double)scan.nextDouble(),
                    theta   = Math.toRadians(scan.nextDouble()),
                    x       = (double)scan.nextDouble(),
                    h1      = (double)scan.nextDouble(),
                    h2      = (double)scan.nextDouble();

            double t = x/(v0*Math.cos(theta));
            double y = v0 * t * Math.sin(theta) - (0.5 * 9.81 * (t*t));
            
           
            if (y > h2 - 1 || y < h1 + 1) {
                System.out.println("Not Safe");
            } else {
                System.out.println("Safe");
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