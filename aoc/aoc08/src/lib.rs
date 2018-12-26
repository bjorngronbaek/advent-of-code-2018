#[cfg(test)]
mod tests {

    #[test]
    fn test_with_no_sub_nodes() {
        let v = vec![0,3,1,2,3];
        let (newindex,serial) = ::sub(&v,0);
        assert_eq!(serial,6);
        assert_eq!(newindex,5);
    }

    #[test]
    fn test_with_single_sub_nodes() {
        let v = vec![1,3,0,2,1,2,1,2,3];
        let (newindex,serial) = ::sub(&v,0);
        assert_eq!(serial,9);
        assert_eq!(newindex,9);
    }

    #[test]
    fn test_with_two_sub_nodes() {
        let v = vec![2,3,0,2,1,2,0,2,3,4,1,2,3];
        let (newindex,serial) = ::sub(&v,0);
        assert_eq!(serial,16);
        assert_eq!(newindex,13);
    }
}

pub fn sub(vec: &Vec<usize>, index: usize) -> (usize,usize) {
    let mut newindex = 0;
    let mut newsum = 0;

    let subnodes = vec.get(index).unwrap();
    let seriallength = vec.get(index+1).unwrap();

    if *subnodes == 0 {
        println!("Found {} subnodes. Start index is:\t{}",subnodes,index);
        let startserial = index + 2;
        let s = sub_calc(vec,startserial,*seriallength);
        newindex = s.0;
        newsum = s.1;
    }

    //let v = vec![1,3,0,2,1,2,1,2,3];
    //             0 1 2 3 4 5 6 7 8
    if *subnodes == 1 {
        let (subnewindex,subsum) = sub(vec,index+2);
        println!("Found {} subnodes. Start index is:\t{}",subnodes,index);
        let startserial = subnewindex;
        let s = sub_calc(vec,startserial,*seriallength);

        newindex = s.0;
        newsum = s.1;            
        newsum += subsum;
    }

    //let v = vec![2,3,0,2,1,2,0,2,3,4,1,2,3];
    //             0 1 2 3 4 5 6 7 8 9 1 1 1
    //                                 0 2 3
    if *subnodes > 1 {
        println!("Found {} subnodes. Start index is:\t{}",subnodes,index);
        let mut loopsum = 0;
        let mut loopindex = index +2;
        for node in 0..*subnodes {
            println!("Processing {} of {} subnodes. Start index is:\t{}",node+1,subnodes,loopindex);
            let (subnewindex,subsum) = sub(vec,loopindex);

            loopindex = subnewindex;
            loopsum += subsum;            
        }

        let s = sub_calc(vec,loopindex,*seriallength);

        newindex = s.0;
        newsum = s.1;
        newsum += loopsum;            
    }

    (newindex,newsum)
}

fn sub_calc(vec: &Vec<usize>, startserial: usize, seriallength: usize) -> (usize,usize) {
    let endserial = startserial + seriallength;

    println!("Start of serial:\t{}",startserial);
    println!("End of serial:\t{}",endserial);

    let newsum = vec[startserial..endserial].iter().sum();
    println!("Sum of serial is:\t{}",newsum);
    let newindex = endserial;

    (newindex,newsum)
}