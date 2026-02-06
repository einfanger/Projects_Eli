import java.util.HashMap;
import java.util.Scanner;

public class BankApp {

    private static HashMap<String, BankAccount> accounts = new HashMap<>();

    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        boolean running = true;

        while (running) {
            printMenu();
            String choice = scanner.nextLine();

            if (choice.equals("1")) {
                createAccount(scanner);
            } else if (choice.equals("2")) {
                deposit(scanner);
            } else if (choice.equals("3")) {
                withdraw(scanner);
            } else if (choice.equals("4")) {
                checkBalance(scanner);
            } else if (choice.equals("5")) {
                listAccounts();
            } else if (choice.equals("0")) {
                running = false;
                System.out.println("Peace ✌️ Thanks for banking with Definitely-Not-a-Scam Bank.");
            } else {
                System.out.println("Invalid choice.");
            }
        }

        scanner.close();
    }

    private static void printMenu() {
        System.out.println("\n=== Simple Banking App ===");
        System.out.println("1) Create Account");
        System.out.println("2) Deposit");
        System.out.println("3) Withdraw");
        System.out.println("4) Check Balance");
        System.out.println("5) List Accounts");
        System.out.println("0) Exit");
        System.out.print("Choose: ");
    }

    private static void createAccount(Scanner scanner) {
        System.out.print("Enter account number: ");
        String accNum = scanner.nextLine();

        if (accounts.containsKey(accNum)) {
            System.out.println("That account number already exists.");
            return;
        }

        System.out.print("Enter owner name: ");
        String name = scanner.nextLine();

        BankAccount account = new BankAccount(accNum, name);
        accounts.put(accNum, account);
        System.out.println("Account created for " + name + " (" + accNum + ")");
    }

    private static BankAccount getAccount(Scanner scanner) {
        System.out.print("Enter account number: ");
        String accNum = scanner.nextLine();

        if (!accounts.containsKey(accNum)) {
            System.out.println("Account not found.");
            return null;
        }
        return accounts.get(accNum);
    }

    private static void deposit(Scanner scanner) {
        BankAccount account = getAccount(scanner);
        if (account == null) return;

        System.out.print("Amount to deposit: ");
        double amount = Double.parseDouble(scanner.nextLine());
        account.deposit(amount);
    }

    private static void withdraw(Scanner scanner) {
        BankAccount account = getAccount(scanner);
        if (account == null) return;

        System.out.print("Amount to withdraw: ");
        double amount = Double.parseDouble(scanner.nextLine());
        account.withdraw(amount);
    }

    private static void checkBalance(Scanner scanner) {
        BankAccount account = getAccount(scanner);
        if (account == null) return;

        System.out.println("Owner: " + account.getOwnerName());
        System.out.println("Balance: $" + account.getBalance());
    }

    private static void listAccounts() {
        if (accounts.isEmpty()) {
            System.out.println("No accounts yet.");
            return;
        }

        System.out.println("\n--- Accounts ---");
        for (String accNum : accounts.keySet()) {
            BankAccount a = accounts.get(accNum);
            System.out.println(accNum + " | " + a.getOwnerName() + " | $" + a.getBalance());
        }
    }
}
