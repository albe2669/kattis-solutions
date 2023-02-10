import java.io.BufferedReader;
import java.io.BufferedOutputStream;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.io.IOException;
import java.util.ArrayList;
import java.util.StringTokenizer;

import java.lang.Math;

public class babylonian {
    static String getStringRepresentation(ArrayList<Character> list) {
        StringBuilder builder = new StringBuilder(list.size());
        for (Character ch : list) {
            builder.append(ch);
        }
        return builder.toString();
    }

    public static void main(String[] args) throws Exception {
        int tc = scan.nextInt();
        for (int i = 0; i < tc; i++) {
            String s = scan.nextLine();
            if (s.charAt(s.length() - 1) == ',') {
                s += "0";
            }
            int j = 0;
            while (j < s.length() - 1) {
                if (s.charAt(j) == ',' && s.charAt(j + 1) == ',') {
                    s = s.substring(0, j + 1) + "0" + s.substring(j + 1, s.length());
                    j++;
                }
                j++;

            }
            String[] strs = s.split(",");
            long l = 0;
            for (j = 0; j < strs.length; j++) {
                if (strs[j] != "0") {
                    long ll = (long) Math.pow(60, (strs.length - 1 - j));
                    l += (ll * Integer.parseInt(strs[j]));
                }
            }
            out.println(l);
        }
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