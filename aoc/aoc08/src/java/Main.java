import java.nio.file.*;
import java.io.*;
import java.util.*;

public class Main {
    
    public static void main(String[] args) {
        Path file = Paths.get(args[0]);
        StringBuilder sb = new StringBuilder();
        try (BufferedReader reader = Files.newBufferedReader(file)) {
            String line = null;
            while ((line = reader.readLine()) != null) {
                sb.append(line);
            }            
        }
        catch (IOException x) {
            System.err.format("IOException: %s%n", x);
        }

        System.out.println(sb.toString());
    }

    public static int getSerialSum(String serial) {
        int serialLength = Integer.valueOf(serial.subString(2, 2));
    }
}