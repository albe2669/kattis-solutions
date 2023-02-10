import java.util.Scanner;

public class lastfactorialdigit {
    private static int f(int n) {
        if (n == 0) return 1; 
        else if (n <= 2) return n; 
        else if (n == 3) return 6; 
        else if (n == 4) return 4; 
        else return 0; 
    }
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        while (sc.hasNextInt()) {
            // Get input
            int n = sc.nextInt();

            for (int i = 0; i < n; i++) {
                System.out.println(f(sc.nextInt()));                
            }            
        }
    }
}