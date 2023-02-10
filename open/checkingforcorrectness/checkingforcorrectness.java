import java.io.*;
import java.math.*;
import java.util.*;

public class checkingforcorrectness {
		public static void main(String[] args) throws Exception {
				Scanner sc = new Scanner(System.in);
				BigInteger modulo = BigInteger.valueOf(10000);

				while (sc.hasNext()) {
						BigInteger a = sc.nextBigInteger();
						char o = sc.next().charAt(0);
						BigInteger b = sc.nextBigInteger();
						
						if (o == '+') {
								System.out.println(a.add(b).mod(modulo));
						} else if (o == '*') {
								System.out.println(a.multiply(b).mod(modulo));
						} else {
								System.out.println(a.modPow(b, modulo));
						}
				}
				sc.close();
		}
}
