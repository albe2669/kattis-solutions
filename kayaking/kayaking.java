import java.io.BufferedReader;
import java.io.BufferedOutputStream;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.io.IOException;
import java.util.StringTokenizer;
import java.util.ArrayList;
import java.util.Collections;

public class kayaking {
    public static void main(String[] args) throws Exception {
        boolean b1 = false;
        boolean n1 = false;
        boolean e1 = false;
        int it = 0;
        int b = scan.nextInt();
        int n = scan.nextInt();
        int e = scan.nextInt();
        int b2 = b;
        int n2 = n;
        int e2 = e;
        
        int sb = scan.nextInt();
        int sn = scan.nextInt();
        int se = scan.nextInt();
        int m = (b+n+e)/2;

        int speedFacMin = 0;
        int speedFacMax = 0;

        for (int i = 0; i < 2; i++) {
            if (b > 0) {
                speedFacMin += sb;
                b--;
            } else if(n > 0) {
                speedFacMin += sn;
                n--;
            } else if(e > 0) {
                speedFacMin += se;
                e--;
            }
        }

        if (e2 > 0) {
            speedFacMax += se;
            e2--;
        } else if(n2 > 0) {
            speedFacMax += sn;
            n2--;
        } else if(b2 > 0) {
            speedFacMax += sb;
            b2--;
        }
        if (b2 > 0) {
            speedFacMax += sb;
            b2--;
        } else if(n2 > 0) {
            speedFacMax += sn;
            n2--;
        } else if(e2 > 0) {
            speedFacMax += se;
            e2--;
        }
    
        ArrayList<Integer> k = new ArrayList<>();
        for(int i = 0; i < m; i++){
            k.add(scan.nextInt());
        }

        int kayakMax = Collections.max(k) * speedFacMin;
        int kayakMin = Collections.min(k) * speedFacMax;

        out.println(kayakMax);
        out.println(kayakMin);

        if (kayakMax > kayakMin) {
            out.println(kayakMax);
        } else if (kayakMin > kayakMax) {
            out.println(kayakMin);
        } else {
            out.println(kayakMin);
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