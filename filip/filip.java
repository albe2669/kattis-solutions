import java.util.Scanner;

public class filip {
    private static int invert(int n) {
        char[] x = String.valueOf(n).toCharArray();
        char[] y = new char[x.length];

        for (int i = x.length; i > 0; i--) {
            y[x.length - i] = x[i - 1];
        }

        return Integer.parseInt(new String(y));
    }

    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        while (sc.hasNextLong()) {
            // Get input
            int a = invert(sc.nextInt());
            int b = invert(sc.nextInt());
            
            if (a > b) {
                System.out.println(a);
            } else {
                System.out.println(b);
            }
        }
    }
}