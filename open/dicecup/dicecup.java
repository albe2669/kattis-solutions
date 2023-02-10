import java.util.Scanner;

public class dicecup {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        while (sc.hasNextLong()) {
            // Get input
            int a = sc.nextInt();
            int b = sc.nextInt();

            if (a > b) {
                int tmp = a;
                a = b;
                b = tmp;
            }
            
            for (int i = a + 1; i < b + 2; i++) {
                System.out.println(i);
            }
        }
    }
}