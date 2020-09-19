import java.io.BufferedReader;
import java.io.BufferedOutputStream;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.io.IOException;
import java.util.StringTokenizer;


public class rockscissorspaper {
    private static char getEnemy(char p) {
        switch (p) {
            case 'R':
                return 'S';                                
            case 'S':
                return 'P';                                
            case 'P':
                return 'R';                                
            default:
                return 'h';
        }
    }
    private static char getWinner(char p, char e) {
        if (getEnemy(p) == e) {
            return p;
        } else if (getEnemy(e) == p){
            return e;
        } else {
            return 'h';
        }
    }
    public static void main(String[] args) throws Exception {
        int tc = scan.nextInt();
        for (int i = 0; i < tc; i++) {
            int r = scan.nextInt(),
                c = scan.nextInt(),
                n = scan.nextInt();

            char[][] field = new char[r][c];

            for (int j = 0; j < r; j++) {
                char[] in = scan.next().toCharArray();
                for (int j2 = 0; j2 < c; j2++) {
                    field[j][j2] = in[j2];
                }
            }

            for (int j = 0; j < n; j++) {
                for (int k = 0; k < field.length; k++) {
                    for (int k2 = 0; k2 < field[j].length; k2++) {
                        char protagonist = field[k][k2];
                        char enemy;

                        if (k - 1 >= 0) {
                            char cell = field[k - 1][k2];
                            char winner = getWinner(protagonist, cell);
                            if (winner == cell) {
                                field[k][k2] = winner;
                            } else if(winner != 'h') {
                                field[k - 1][k2] = winner;
                            }

                        }

                        if (k + 1 < field.length) {
                            char cell = field[k + 1][k2];
                            char winner = getWinner(protagonist, cell);
                            if (winner == cell) {
                                field[k][k2] = winner;
                            } else if(winner != 'h') {
                                field[k + 1][k2] = winner;
                            }
                        }

                        if (k2 - 1 >= 0) {
                            char cell = field[k][k2 - 1];
                            char winner = getWinner(protagonist, cell);
                            if (winner == cell) {
                                field[k][k2] = winner;
                            } else if(winner != 'h') {
                                field[k][k2 - 1] = winner;
                            }
                        }

                        if (k2 + 1 < field[k].length) {
                            char cell = field[k][k2 + 1];
                            char winner = getWinner(protagonist, cell);
                            if (winner == cell) {
                                field[k][k2] = winner;
                            } else if(winner != 'h') {
                                field[k][k2 + 1] = winner;
                            }
                        }
                    }
                }
            }

            for (int j = 0; j < field.length; j++) {
                out.println(new String(field[j]));
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