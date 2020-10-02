import java.io.BufferedReader;
import java.io.BufferedOutputStream;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.io.IOException;
import java.util.HashMap;
import java.util.StringTokenizer;


public class forestfruits {
    public static 
    public static void main(String[] args) throws Exception {
        int v = scan.nextInt();
        int e = scan.nextInt();
        int c = scan.nextInt();
        int k = scan.nextInt();
        int m = scan.nextInt();
        
        int[][] trails = new int[e][3];
        HashMap<Integer, int[]> clearings = new HashMap<Integer, int[]>();
        int[] fruits = new int[c];

        for (int i = 0; i < e; i++) {
            trails[i][0] = scan.nextInt();
            trails[i][1] = scan.nextInt();
            trails[i][2] = scan.nextInt();
        }

        for (int i = 0; i < c; i++) {
            fruits[i] = scan.nextInt();
        }

        for (int i = 0; i < trails.length; i++) {
            int ui = trails[i][0];
            int vi = trails[i][1];
            int wi = trails[i][2];



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