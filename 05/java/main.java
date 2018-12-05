import java.nio.file.*;
import java.io.*;
import java.util.*;

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
        /* 05-1
        main searcher = new main(s);
        searcher.search(s);
        */

        HashSet<String> units = new HashSet<String>();
        for (char ch: s.toCharArray()) {
            units.add(Character.toString(ch).toLowerCase());
        }

        int unitcount = s.length();
        for (String temp : units) {
            String s1 = s.replace(temp,"");
            String s2 = s1.replace(temp.toUpperCase(),"");
            System.out.println("Searching "+temp);
            main searcher = new main(s2);
            int c = searcher.search(s2);
            if(c < unitcount){
                System.out.println("updating");
                unitcount = c;
            }            
        }
        System.out.println("Min count "+unitcount);
    }

    private String s;
    public main(String s){
        this.s = s;
    }

    public int search(String s){
        System.out.println("Searching "+s);
        String pair = findPair(s);
        if(pair != null){
            System.out.println("Found "+pair);
            s = s.replace(pair,"");
            return search(s);
        }
        else{
            //System.out.println(s);
            System.out.println("Found "+s.length()+" units");
        }
        return s.length();
    }

    public String findPair(String s){
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