import java.util.Scanner;
import java.lang.Math;
import java.util.Arrays;

public class pot {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        while (sc.hasNextLong()) {
            // Get input
            long    n = sc.nextLong();

            long total = 0;
            for (int i = 0; i < n; i++) {
                long p = sc.nextLong();

                char[] number = String.valueOf(p).toCharArray();
                int pow = Integer.parseInt(String.valueOf(number[number.length-1]));
                total += Math.pow(Double.parseDouble(new String(Arrays.copyOfRange(number, 0, number.length-1))), pow);
            }
            
            System.out.println(total);
        }
    }
}