import java.math.BigInteger;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Arrays;

//TIP To <b>Run</b> code, press <shortcut actionId="Run"/> or
// click the <icon src="AllIcons.Actions.Execute"/> icon in the gutter.
public class Main {
    public static void main(String[] args) {
        //TIP Press <shortcut actionId="ShowIntentionActions"/> with your caret at the highlighted text
        // to see how IntelliJ IDEA suggests fixing it.
        System.out.println("Hello and welcome!");
        try {
            Path filePath = Path.of("inputs/input.txt");
            String[] lines = Files.readAllLines(filePath).toArray(String[]::new);
            lines[0] = lines[0].replace('S', '|');
            // Initialize a bigint matrix of the same size
            BigInteger[][] matrix = new BigInteger[lines.length][lines[0].length()];
            for (BigInteger[] bigIntegers : matrix) {
                Arrays.fill(bigIntegers, BigInteger.ZERO);
            }
            matrix[0] = lines[0].chars().mapToObj(c -> c == '|' ? BigInteger.ONE : BigInteger.ZERO).toArray(BigInteger[]::new);
            long numberOfSplits = 0;
            for (int i = 1; i < lines.length; i++) {
                Character[] line = lines[i].chars()
                        .mapToObj(c -> (char) c)
                        .toArray(Character[]::new);
                for (int j = 0; j < line.length; j++) {
                    if (lines[i-1].charAt(j) == '|') {
                        if (line[j] == '^') {
                            if (j > 0) {
                                line[j - 1] = '|';
                                matrix[i][j - 1] = matrix[i][j - 1].add(matrix[i - 1][j]);
                            }
                            if (j < line.length - 1) {
                                line[j + 1] = '|';
                                matrix[i][j + 1] = matrix[i][j + 1].add(matrix[i - 1][j]);
                            }
                            numberOfSplits++;
                        }
                        else {
                            line[j] = '|';
                            matrix[i][j] = matrix[i][j].add(matrix[i - 1][j]);
                        }
                    }
                }
                StringBuilder newLine = new StringBuilder();
                for (Character c : line) {
                    newLine.append(c);
                }
                System.out.println(newLine);
                lines[i] = newLine.toString();
            }
            System.out.println("Number of splits: " + numberOfSplits);
            BigInteger totalWays = BigInteger.ZERO;
            for (BigInteger bigInteger : matrix[matrix.length - 1]) {
                totalWays = totalWays.add(bigInteger);
            }
            System.out.println("Total ways to reach the bottom: " + totalWays);
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}