#[cfg(test)]
mod tests {

    #[test]
    fn test_with_no_sub_nodes() {
        let v = vec![0,3,1,2,3];
        let n = ::sub(&v,0);
        assert_eq!(n.newsum,6);
        assert_eq!(n.newindex,5);
        assert_eq!(n.newvalue,6);
    }

    #[test]
    fn test_with_single_sub_nodes() {
        let v = vec![1,3,0,2,1,2,1,2,3];
        let n = ::sub(&v,0);
        assert_eq!(n.newsum,9);
        assert_eq!(n.newindex,9);
        assert_eq!(n.newvalue,3);
    }

    #[test]
    fn test_with_two_sub_nodes() {
        let v = vec![2,3,0,2,1,2,0,2,3,4,1,2,3];
        let n = ::sub(&v,0);
        assert_eq!(n.newsum,16);
        assert_eq!(n.newindex,13);
        assert_eq!(n.newvalue,10);
    }
}

#[macro_use]
extern crate log;

pub struct NodeValue {
    pub newindex: usize,
    pub newsum: usize,
    pub newvalue: usize,
}

impl NodeValue {
    pub fn new(index: usize,sum: usize,value: usize) -> NodeValue {
        NodeValue {
            newindex: index,
            newsum: sum,
            newvalue: value,
        }
    }
}

pub fn sub(vec: &Vec<usize>, index: usize) -> NodeValue {
    error!("test");

    let mut newindex = 0;
    let mut newsum = 0;
    let mut newvalue = 0;

    let subnodes = vec.get(index).unwrap();
    let seriallength = vec.get(index+1).unwrap();

    if *subnodes == 0 {
        println!("Found {} subnodes. Start index is:\t{}",subnodes,index);
        let startserial = index + 2;
        let s = sub_calc(vec,startserial,*seriallength,&Vec::new());
        newindex = s.0;
        newsum = s.1;
        newvalue = newsum;
    }

    //let v = vec![1,3,0,2,1,2,1,2,3];
    //             0 1 2 3 4 5 6 7 8
    if *subnodes == 1 {
        let nodevalue = sub(vec,index+2);
        println!("Found {} subnodes. Start index is:\t{}",subnodes,index);
        let startserial = nodevalue.newindex;
        
        
        let s = sub_calc(vec,startserial,*seriallength,&Vec::new());

        newindex = s.0;
        newsum += s.1;            
        newsum += nodevalue.newsum;

        let endserial = startserial + *seriallength;
        for i in vec[startserial..endserial].iter() {
            if *i == 1 {
                newvalue += nodevalue.newsum;
            }
        }
    }

    //let v = vec![2,3,0,2,1,2,0,2,3,4,1,2,3];
    //             0 1 2 3 4 5 6 7 8 9 1 1 1
    //                                 0 2 3
    if *subnodes > 1 {
        println!("Found {} subnodes. Start index is:\t{}",subnodes,index);
        let mut loopsum = 0;
        let mut loopindex = index +2;

        let mut nodevalues = Vec::new();
        for node in 0..*subnodes {
            println!("Processing {} of {} subnodes. Start index is:\t{}",node+1,subnodes,loopindex);
            let nodevalue = sub(vec,loopindex);
            loopindex = nodevalue.newindex;
            loopsum += nodevalue.newsum;            
            nodevalues.push(nodevalue);            
        }

        let s = sub_calc(vec,loopindex,*seriallength,&nodevalues);

        newindex = s.0;
        newsum = s.1;
        newsum += loopsum;            
        newvalue = s.2;
    }

    NodeValue::new(newindex,newsum,newvalue)
}

fn sub_calc(vec: &Vec<usize>, startserial: usize, seriallength: usize, subnodevalues: &Vec<NodeValue>) -> (usize,usize,usize) {
    let endserial = startserial + seriallength;

    println!("Start of serial:\t{}",startserial);
    println!("End of serial:\t{}",endserial);

    //let newsum = vec[startserial..endserial].iter().sum();
    let mut newsum = 0;
    let mut newvalue = 0;
    for value in vec[startserial..endserial].iter() {
        newsum += value;
        match subnodevalues.get(*value-1) {
            Some(v) => newvalue += v.newvalue,
            None => newvalue += 0,
        }
    }

    println!("Sum of serial is:\t{}",newsum);
    println!("Sum of values is:\t{}",newvalue);
    let newindex = endserial;

    (newindex,newsum,newvalue)
}