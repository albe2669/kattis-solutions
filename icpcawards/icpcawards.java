import java.io.BufferedReader;
import java.io.BufferedOutputStream;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.io.IOException;
import java.util.StringTokenizer;
import java.util.LinkedHashMap;
import java.util.Map;

public class icpcawards {
    public static void main(String[] args) throws Exception {
        int tc = scan.nextInt();
        LinkedHashMap<String, String> c = new LinkedHashMap<String, String>();

        for (int i = 0; i < tc; i++) {
            String uni = scan.next();
            String team = scan.next();

            if (c.containsKey(uni)) continue;
            
            c.put(uni, team);
        }

        int i = 0;
        for (Map.Entry<String, String> entry : c.entrySet()) {
            i++;

            if (i == 13) {
                break;
            }
            System.out.println(entry.getKey() + " " + entry.getValue());
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