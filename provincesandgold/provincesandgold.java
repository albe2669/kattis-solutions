import java.io.BufferedReader;
import java.io.BufferedOutputStream;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.io.IOException;
import java.util.StringTokenizer;


public class provincesandgold {
    public static void main(String[] args) throws Exception {
        int buyingPower = scan.nextInt() * 3 + scan.nextInt() * 2 + scan.nextInt();
        
        if (buyingPower >= 8) {
            out.print("Province or ");
        } else if (buyingPower >= 5) {
            out.print("Duchy or ");
        } else if (buyingPower >= 2) {
            out.print("Estate or ");
        }

        if (buyingPower >= 6) {
            out.println("Gold");
        } else if (buyingPower >= 3) {
            out.println("Silver");
        } else {
            out.println("Copper");
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