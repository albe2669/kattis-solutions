import java.util.Scanner;

import java.lang.Math;

public class humancannonball2 {
    public static void main(String[] args) throws Exception {
        Scanner scan = new Scanner(System.in);

        int n = scan.nextInt();
        for (int i = 0; i < n; i++) {
            if (i != 1) {
                continue;
            }
            double  v0      = (double)scan.nextDouble(),
                    theta   = Math.toRadians(scan.nextDouble()),
                    x       = (double)scan.nextDouble(),
                    h1      = (double)scan.nextDouble(),
                    h2      = (double)scan.nextDouble();

            double t = x/(v0*Math.cos(theta));
            double y = v0 * t * Math.sin(theta) - (0.5 * 9.81 * (t*t));
            
            System.out.println(v0 * t * Math.sin(theta));
            System.out.println((0.5 * 9.81 * (double)Math.pow(t, 2)));
            
            System.out.println(t);
            System.out.print(y + " - ");
            if (y > h2 - 1 || y < h1 + 1) {
                System.out.println("Not Safe");
            } else {
                System.out.println("Safe");
            }
        }
    }
}