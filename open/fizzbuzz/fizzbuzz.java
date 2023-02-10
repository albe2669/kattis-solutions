import java.util.Scanner;

public class fizzbuzz {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        while (sc.hasNextLong()) {
            // Get input
            int x = sc.nextInt(),
                y = sc.nextInt(),
                n = sc.nextInt();
            
                for (int i = 1; i <= n; i++) {
                    String out = "";

                    if (i % x == 0) {
                        out += "Fizz";
                    }

                    if (i % y == 0) {
                        out += "Buzz";
                    }

                    if (out != "") {
                        System.out.println(out);
                    } else {
                        System.out.println(i);
                    }
                }
                     
        }
    }
}