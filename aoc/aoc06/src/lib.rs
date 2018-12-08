extern crate regex;
use regex::Regex;

#[cfg(test)]
mod tests {

    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[test]
    fn test_point_m_distance() {
        let &p1 = ::Point::new(0,0);
        assert_eq!(0,p1.m_distance(&::Point::new(0,0)));
        assert_eq!(2,p1.m_distance(&::Point::new(1,1)));
        assert_eq!(4,p1.m_distance(&::Point::new(2,2)));

        let &p2 = ::Point::new(-1,-1);
        assert_eq!(4,p2.m_distance(&::Point::new(1,1)));
        assert_eq!(0,p2.m_distance(&::Point::new(-1,-1)));
        assert_eq!(2,p2.m_distance(&::Point::new(-2,-2)));
    }

    #[test]
    fn test_hash() {        
        assert_eq!(calculate_hash(&::Point::new(1,1)),calculate_hash(&::Point::new(1,1)));
        assert_ne!(calculate_hash(&::Point::new(1,0)),calculate_hash(&::Point::new(0,1)));
    }

    #[test]
    fn test_eq() {        
        assert_eq!(&::Point::new(1,1),&::Point::new(1,1));
        assert_ne!(&::Point::new(1,0),&::Point::new(0,1));
    }

    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}

#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {

    pub fn new(x: i32, y: i32) -> Point {
        Point{
            x: x,
            y: y,
        }
    }

    pub fn parse(line: &str) -> Point {
        let re = Regex::new(r"^(\d+),\s(\d+)$").unwrap();
        let coordinates = re.captures(line).unwrap();
        Point{
            x: coordinates.get(1).unwrap().as_str().parse().unwrap(),
            y: coordinates.get(2).unwrap().as_str().parse().unwrap(),
        }
    }

    pub fn m_distance(&self, p: &Point) -> usize {
        ((self.x - p.x).abs() + (self.y - p.y).abs()) as usize
    }

    pub fn find_borders(points: &Vec<Point>) -> (Point,Point) {
        let mut extreem_left = std::i32::MAX;
        let mut extreem_right = std::i32::MIN;
        let mut extreem_bottom = std::i32::MIN;
        let mut extreem_top = std::i32::MAX;
        
        for point in points {
            if point.x < extreem_left {
                extreem_left = point.x;
            }

            if point.x > extreem_right {
                extreem_right = point.x;
            }

            if point.y > extreem_bottom {
                extreem_bottom = point.y;
            }

            if point.y < extreem_top {
                extreem_top = point.y;
            }
        }

        (Point::new(extreem_left,extreem_top),Point::new(extreem_right,extreem_bottom))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

/*
impl Hash for Point {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        state.write_u32(self.x);
        state.write_u32(self.y);
        state.finish();
    }
}
*/