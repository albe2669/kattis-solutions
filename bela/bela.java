import java.io.BufferedReader;
import java.io.BufferedOutputStream;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.io.IOException;
import java.util.StringTokenizer;


public class bela {
    public static void main(String[] args) throws Exception {
        int n = scan.nextInt();
        char dominant = scan.next().toCharArray()[0];

        int total = 0;
        for (int i = 0; i < 4*n; i++) {
            char[] card = scan.next().toCharArray();
            switch (card[0]) {
                case 'A':
                    total += 11;
                    break;
            
                case 'K':
                    total += 4;
                    break;

                case 'Q':
                    total += 3;
                    break;

                case 'J':
                    if (card[1] == dominant) {
                        total += 20;
                    } else {
                        total += 2;
                    }
                    break;

                case 'T':
                    total += 10;
                    break;

                case '9':
                    if (card[1] == dominant) {
                        total += 14;
                    }
                    break;
                default:
                    break;

            }
        }
        out.println(total);
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