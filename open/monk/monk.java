import java.io.BufferedReader;
import java.io.BufferedOutputStream;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.io.IOException;
import java.util.StringTokenizer;


public class monk {
    public static void main(String[] args) throws Exception {
        int a = scan.nextInt();
        int d = scan.nextInt();


        int[][] ascend = new int[a][2];
        for (int i = 0; i < a; i++) {
            ascend[i] = new int[]{scan.nextInt(), scan.nextInt()};
        }

        int[][] descend = new int[d][2];
        for (int i = 0; i < d; i++) {
            descend[i] = new int[]{scan.nextInt(), scan.nextInt()};
        }

        double x = 0;
        double y = 0;

        for (int i = 0; i < descend.length; i++) {
            double  descendH0 = descend[i][0],
                    descendH1 = descend[i][0],
                    descendT0 = descend[i][1],
                    descendT1 = descend[i][1];

            if (i + 1 > descend.length - 1) {
                descendH1 = 0;
                descendT1 = 0;
            } else {
                descendH1 = descend[i+1][0];
                descendT1 = descend[i+1][1];
            }
            
            for (int j = 0; j < ascend.length; j++) {
                double  ascendH0 = ascend[j][0],
                        ascendH1 = ascend[j][0],
                        ascendT0 = ascend[j][1],
                        ascendT1 = ascend[j][1];

                if (j + 1 > ascend.length - 1) {
                    ascendH0 = 0;
                    ascendT0 = 0;
                    ascendH1 = ascend[j][0];
                    ascendT1 = ascend[j][1];
                } else {
                    ascendH1 = ascend[i+1][0];
                    ascendT1 = ascend[i+1][1];
                }
                

                double ascendA = (ascendH1 - ascendH0)/(ascendT1 - ascendT0);
                double ascendB = (ascendH1 - ascendA * ascendT1);

                double descendA = (descendH1 - descendH0)/(descendT1 - descendT0);
                double descendB = (descendH0 - descendA * descendT0);

                out.println("ascendH0: " + ascendH0);
                out.println("ascendH1: " + ascendH1);
                out.println("ascendT0: " + ascendT0);
                out.println("ascendT1: " + ascendT1);
                
                out.println("descendH0: " + descendH0);
                out.println("descendH1: " + descendH1);
                out.println("descendT0: " + descendT0);
                out.println("descendT1: " + descendT1);
                
                out.println("ascendA: " + ascendA);
                out.println("ascendB: " + ascendB);
                out.println("descendA: " + descendA);
                out.println("descendB: " + descendB);

                x = (ascendB - descendB)/(ascendA-descendA);
                y = ascendA * x + ascendB;
                
                out.println(x);
                out.println(y);
            }
        }

        out.println(x);
        out.println(y);
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