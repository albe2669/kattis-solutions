import java.util.Scanner;

public class qaly {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        while (sc.hasNextLong()) {
            // Get input
            long    a = sc.nextLong();

            double total = 0;
            for (int i = 0; i < a; i++) {
                total += sc.nextDouble() * sc.nextDouble();
            }
            
            System.out.println(total);
        }
    }
}