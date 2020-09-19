import java.util.Scanner;

public class heartrate {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        while (sc.hasNextLong()) {
            // Get input
            int n = sc.nextInt();
            
            for (int i = 0; i < n; i++) {
                int b = sc.nextInt();
                double p = sc.nextDouble();

                double bpm = (60 * b)/p;
                System.out.println(String.valueOf(bpm - (60/p)) + " " + String.valueOf(bpm) + " " + String.valueOf(bpm + (60/p)));
            }
        }
    }
}