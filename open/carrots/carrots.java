import java.util.Scanner;

public class carrots {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        while (sc.hasNextLong()) {
            // Get input
            long    n = sc.nextLong(), 
                    p = sc.nextLong();
            
            for (int i = 0; i < n; i++) {
                sc.nextLine();
            }

            System.out.println(p);
        }
    }
}