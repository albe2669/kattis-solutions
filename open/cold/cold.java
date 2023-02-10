import java.util.Scanner;

public class cold {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        while (sc.hasNextLong()) {
            // Get input
            int n = sc.nextInt();

            int t = 0;
            for (int i = 0; i < n; i++) {
                if (sc.nextInt() < 0) {
                    t++;
                }
            }
            
            System.out.println(t);
        }
    }
}