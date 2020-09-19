import java.util.Scanner;
import java.lang.Math;

public class faktor {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        while (sc.hasNextLong()) {
            // Get input
            long a = sc.nextLong();
            double i = sc.nextLong();
            
            i = i-0.9999;

            long out = (long)Math.ceil(i*a);

            System.out.println(out);
        }
    }
}