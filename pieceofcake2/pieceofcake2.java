import java.util.Scanner;

public class pieceofcake2 {
    private static long calculate(int h, int w) {
        return h * w * 4;
    }

    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        while (sc.hasNextInt()) {
            // Get input
            int n = sc.nextInt(), 
                h = sc.nextInt(),
                v = sc.nextInt();
            
            long fi = calculate(h, v);
            long se = calculate(h, n-v);
            long th = calculate(n-h, v);
            long ft = calculate(n-h, n-v);

            if (fi > se && fi > th && fi > ft) {
                System.out.println(fi);
            } else if (se > fi && se > th && se > ft) {
                System.out.println(se);
            } else if (th > se && th > fi && th > ft) {
                System.out.println(th);
            } else {
                System.out.println(ft);
            }
        }
    }
}