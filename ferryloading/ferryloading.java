import java.io.BufferedReader;
import java.io.BufferedOutputStream;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.io.IOException;
import java.util.StringTokenizer;
import java.util.ArrayList;

public class ferryloading {
    public static void main(String[] args) throws Exception {
        int ferry = scan.nextInt() * 100;
        int port = 0;
        int starboard = 0;
        int boarded = 0;
        ArrayList<Integer> cars  = new ArrayList<Integer>();
        ArrayList<String> board = new ArrayList<String>();
        
        while(true) {
            int current = scan.nextInt();
            if (current == 0) {
                break;
            }

            cars.add(current);
        }

        for (Integer car : cars) {
            if (port + car <= ferry) {
                port += car;
                boarded++;
                board.add("port");
            } else if (starboard + car <= ferry) {
                starboard += car;
                boarded++;
                board.add("starboard");
            }
        }

        System.out.println(boarded);

        for (String string : board) {
            System.out.println(string);
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