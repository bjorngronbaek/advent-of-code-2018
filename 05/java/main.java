import java.nio.file.*;
import java.io.*;

public class main{

    public static void main(String[] args){
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

        String s = sb.toString();    
        search(s);
    }

    public static void search(String s){
        System.out.println("Searching "+s);
        String pair = findPair(s);
        if(pair != null){
            System.out.println("Found "+pair);
            s = s.replace(pair,"");
            search(s);
        }
        else{
            System.out.println(s);
            System.out.println("Found "+s.length()+" units");
        }
    }

    public static String findPair(String s){
        for(int i=0; i<s.length()-1;i++){
            String first = Character.toString(s.charAt(i));
            String second = Character.toString(s.charAt(i+1));
            if(!first.equals(second) && (first.toUpperCase().equals(second) || first.toLowerCase().equals(second))){
                return first+second;                    
            }                                
        }   
        return null;
    }
}