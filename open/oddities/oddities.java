import java.util.Scanner;

public class oddities {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        while (sc.hasNextLong()) {
            // Get input
            int n = sc.nextInt();
            
            for (int i = 0; i < n; i++) {
                int in = sc.nextInt();

                if (in % 2 == 0){
                    System.out.println(String.valueOf(in) + " is even");
                } else {
                    System.out.println(String.valueOf(in) + " is odd");
                }
            }
                     
        }
    }
}