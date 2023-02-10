import java.util.Scanner;

public class nastyhacks {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        while (sc.hasNextLong()) {
            // Get input
            int n = sc.nextInt();

            for (int i = 0; i < n; i++) {
                int r = sc.nextInt(),
                    e = sc.nextInt(),
                    c = sc.nextInt();

                int adv = e-c;

                if (r < adv) {
                    System.out.println("advertise");
                } else if (r > adv) {
                    System.out.println("do not advertise");
                } else {
                    System.out.println("does not matter");
                }
            }          
        }
    }
}