import java.io.BufferedReader;
import java.io.BufferedOutputStream;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.io.IOException;
import java.util.StringTokenizer;

import java.lang.Math;

public class commonground {
    
    static int[] reduceFraction(int x, int y)  
    {  
        int d;  
        d = gcd(x, y);  
    
        x = x / d;  
        y = y / d;  
    
        return new int[]{x, y};
    }  
    
    static int gcd(int a, int b)  
    {  
        if (b == 0)  
            return a;  
        return gcd(b, a % b);  
        
    } 
    public static void main(String[] args) throws Exception {
        int n = scan.nextInt();
        int l = scan.nextInt();
        int w = scan.nextInt();

        int[][] arena = new int[l][w];

        int common = 0;
        int pony = 0;
        int quibble = 0;
        int empty = 0;
    

        for (int i = 0; i < n; i++) {
            int x = scan.nextInt();
            int y = scan.nextInt();
            int r = scan.nextInt();

            for (int j = 0; j < arena.length; j++) {
                for (int j2 = 0; j2 < arena[j].length; j2++) {
                    if (Math.abs(x-j) + Math.abs(y-j2) <= r ) {
                        if (arena[j][j2] != 1) {
                            pony++;
                            arena[j][j2] = 1;
                        }
                    }
                }
            }
        }
        
        for (int i = 0; i < n; i++) {
            int x = scan.nextInt();
            int y = scan.nextInt();
            int r = scan.nextInt();

            for (int j = 0; j < arena.length; j++) {
                for (int j2 = 0; j2 < arena[j].length; j2++) {
                    if (Math.abs(x-j) + Math.abs(y-j2) <= r ) {
                        if (arena[j][j2] == 1) {
                            arena[j][j2] = 3;
                            quibble++;
                            common++;
                        } 
                        if (arena[j][j2] != 3 && arena[j][j2] != 2) {
                            arena[j][j2] = 2;
                            quibble++;
                        }
                    }
                }
            }
        }

        for (int j = 0; j < arena.length; j++) {
            for (int j2 = 0; j2 < arena[j].length; j2++) {
                if (arena[j][j2] == 0) {
                    empty++;
                }
            }
        }

        out.println("Common: " + common);
        out.println("Pony: " + pony);
        out.println("Quibble: " + quibble);
        out.println("Empty: " + empty);
        out.println("Arena: " + l * w);

        int[] cp = reduceFraction(common, pony);
        int[] cq = reduceFraction(common, quibble);
        int[] cl = reduceFraction(common, l*w);
        int[] ce = reduceFraction(common, empty);
        out.println("Common / Pony: "       + cp[0] + "/" + cp[1]);
        out.println("Common / Quibble: "    + cq[0] + "/" + cq[1]);
        out.println("Common / Arena: "      + cl[0] + "/" + cl[1]);
        out.println("Common / Empty: "      + ce[0] + "/" + ce[1]);
        out.close();
    }

    static class Point 
{ 
    int x, y; 
  
    public Point(int x, int y)  
    { 
        this.x = x; 
        this.y = y; 
    } 
}; 
  
    static int overlappingArea(Point l1, Point r1, 
                            Point l2, Point r2) 
    { 
        int area1 = Math.abs(l1.x - r1.x) * 
                    Math.abs(l1.y - r1.y); 
    
        int area2 = Math.abs(l2.x - r2.x) * 
                    Math.abs(l2.y - r2.y); 
    
        int areaI = (Math.min(r1.x, r2.x) -  
                    Math.max(l1.x, l2.x)) *  
                    (Math.min(r1.y, r2.y) - 
                    Math.max(l1.y, l2.y)); 
    
        return (area1 + area2 - areaI); 
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