#[cfg(test)]
mod tests {

    //extern crate ../aoc08;
    
    #[test]
    fn test_string_splitter() {
        let serial = String::from("0 3");
        let v = serial.split(" ").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        assert_eq!(2, v.len());
    }

    #[test]
    fn test_string_to_usize_splitter() {
        let serial = String::from("0 3");
        let v = serial.split(" ").collect::<Vec<&str>>();
        assert_eq!(2, v.len());
    }

    #[test]
    fn test_simple_splitter() {
        let serial = String::from("0 3 1 2 3");
        let v = serial.split(" ").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        assert_eq!(::aoc08::split(v), vec![1,2,3]);
    }
}

pub fn split(serial: Vec<usize>) -> Vec<usize> {    
    if serial.get(0).unwrap() == 0 {
        return serial.get(1..*serial.get(1).unwrap()).unwrap();
    }

    serial
}

pub fn split_into_nodes(nodes: Vec<&str>) -> usize {
    let mut serial_sum = 0;
    if *nodes.get(0).unwrap() == "0" {
         let serial_length: usize = nodes.get(1).unwrap().parse().unwrap();
         let serials = nodes.get(2..serial_length);
         
      
         for serial in serials {
             println!("{:?}",serial);
            //let s: usize = serial.parse().unwrap();
            //serial_sum += s;
         }
    }

    serial_sum
}