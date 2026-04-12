import java.util.Scanner;

public class name {
    public static void main (String[] args) {
        Scanner input = new Scanner (System.in);
        System.out.print("Enter Your Name : ");
        String name = input.nextLine();
        System.out.print(name);
        input.close();
    }
}
