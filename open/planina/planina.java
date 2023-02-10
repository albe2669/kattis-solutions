import java.util.Scanner;
import java.lang.Math;

public class planina {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        while (sc.hasNextLong()) {
            // Get input
            long    a = sc.nextLong();

            System.out.println((long)Math.pow(Math.pow(2, a)+ 1, 2));
        }
    }
}