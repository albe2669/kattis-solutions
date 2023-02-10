import java.util.Scanner;

public class grassseed {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        while (sc.hasNextDouble()) {
            double  c = sc.nextDouble(),
                    l = sc.nextDouble();

            double total = 0;

            for (int i = 0; i < l; i++) {
                double  w = sc.nextDouble(),
                        h = sc.nextDouble();

                total += (w * h) * c;
            }
            
            System.out.println(total);
        }
    }
}