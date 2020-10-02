import java.io.BufferedReader;
import java.io.BufferedOutputStream;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.io.IOException;
import java.util.StringTokenizer;
import java.util.ArrayList;

public class display {
    public static String[] getNumber(int n) {
        String[] num;

        switch (n) {
            case 1:
                num = new String[]
                    {
                        "    +",
                        "    |",
                        "    |",
                        "    +",
                        "    |",
                        "    |",
                        "    +",
                    };
                break;
            case 2:
                num = new String[]
                    {
                        "+---+",
                        "    |",
                        "    |",
                        "+---+",
                        "|    ",
                        "|    ",
                        "+---+",
                    };
                break;
            case 3:
                num = new String[]
                    {
                        "+---+",
                        "    |",
                        "    |",
                        "+---+",
                        "    |",
                        "    |",
                        "+---+",
                    };
                break;
            case 4:
                num = new String[]
                    {
                        "+   +",
                        "|   |",
                        "|   |",
                        "+---+",
                        "    |",
                        "    |",
                        "    +",
                    };
                break;
            case 5:
                num = new String[]
                    {
                        "+---+",
                        "|    ",
                        "|    ",
                        "+---+",
                        "    |",
                        "    |",
                        "+---+",
                    };
                break;
            case 6:
                num = new String[]
                    {
                        "+---+",
                        "|    ",
                        "|    ",
                        "+---+",
                        "|   |",
                        "|   |",
                        "+---+",
                    };
                break;
            case 7:
                num = new String[]
                    {
                        "+---+",
                        "    |",
                        "    |",
                        "    +",
                        "    |",
                        "    |",
                        "    +",
                    };
                break;
            case 8:
                num = new String[]
                    {
                        "+---+",
                        "|   |",
                        "|   |",
                        "+---+",
                        "|   |",
                        "|   |",
                        "+---+",
                    };
                break;
            case 9:
                num = new String[]
                    {
                        "+---+",
                        "|   |",
                        "|   |",
                        "+---+",
                        "    |",
                        "    |",
                        "+---+",
                    };
                break;
            default:
                num = new String[]
                    {
                        "+---+",
                        "|   |",
                        "|   |",
                        "+   +",
                        "|   |",
                        "|   |",
                        "+---+",
                    };
                break;
        }

        return num;
    }

    public static void main(String[] args) throws Exception {
        ArrayList<String> times = new ArrayList<String>();

        while(true) {
            String current = scan.next();
            if (current.equals("end")) {
                break;
            }

            times.add(current);
        }

        for (String time : times) {
            char[] t = time.toCharArray();
            int n = 0;
            String[][] display = new String[4][7];

            for (char l : t) {
                if (l == ':') continue;

                display[n] = getNumber(Integer.parseInt(String.valueOf(l)));
                n += 1;
            }

            for (int i = 0; i < 7; i++) {
                System.out.print(display[0][i] + "  ");
                System.out.print(display[1][i] + "  ");
                
                if (i == 2 || i == 4) {
                    System.out.print("o  ");
                } else {
                    System.out.print("   ");
                }

                System.out.print(display[2][i] + "  ");
                System.out.println(display[3][i]);
            }
            System.out.println("");
            System.out.println("");
        }

        System.out.println("end");
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