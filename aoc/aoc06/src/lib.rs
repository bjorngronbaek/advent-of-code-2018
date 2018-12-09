extern crate regex;
use regex::Regex;
use std::collections::HashSet;

#[cfg(test)]
mod tests {

    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::collections::HashSet;

    #[test]
    fn test_point_m_distance() {
        let p1 = ::Point::new(0,0);
        assert_eq!(0,p1.m_distance(&::Point::new(0,0)));
        assert_eq!(2,p1.m_distance(&::Point::new(1,1)));
        assert_eq!(4,p1.m_distance(&::Point::new(2,2)));

        let p2 = ::Point::new(-1,-1);
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

    #[test]
    fn test_closest(){
        let target = ::Point::new(0,0);
        let p1 = ::Point::new(1,1);
        let p2 = ::Point::new(2,2);
        let mut points: HashSet<::Point> = HashSet::new();
        points.insert(p1);
        points.insert(p2);

        assert_eq!(::Point::new(1,1), target.find_closest_point(&points).unwrap());
        assert_eq!(::Point::new(1,1), ::Point::new(1,1).find_closest_point(&points).unwrap());
        assert_eq!(::Point::new(2,2), ::Point::new(3,3).find_closest_point(&points).unwrap());
        assert_ne!(::Point::new(1,1), ::Point::new(3,3).find_closest_point(&points).unwrap());

        assert_eq!(::Point::new(2,1).find_closest_point(&points).is_none(),true);
    }

    #[test]
    fn test_totaldistance(){
        let target = ::Point::new(0,0);
        let p1 = ::Point::new(1,1);
        let p2 = ::Point::new(2,2);
        let mut points: HashSet<::Point> = HashSet::new();
        points.insert(p1);
        points.insert(p2);

        assert_eq!(6, target.find_total_distance(&points));
    }

    #[test]
    fn test_closest_specific(){
        let target = ::Point::new(4,1);
        let p1 = ::Point::new(1,1);
        let p2 = ::Point::new(3,4);
        let mut points: HashSet<::Point> = HashSet::new();
        points.insert(p1);
        points.insert(p2);

        assert_eq!(::Point::new(1,1), target.find_closest_point(&points).unwrap());
    }

    #[test]
    fn test_distance_specific(){
        let target = ::Point::new(4,1);
        let p1 = ::Point::new(1,1);
        let p2 = ::Point::new(3,4);

        assert_eq!(3,p1.m_distance(&target));
        assert_eq!(4,p2.m_distance(&target));
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
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

    pub fn find_borders(points: &HashSet<Point>) -> (Point,Point) {
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

    pub fn find_closest_point(&self, points: &HashSet<Point>) -> Option<Point> {
        let mut closest_dist = std::usize::MAX;
        let mut closest_point = Point::new(0,0);
        let mut closest_point_count = 0;
        for point in points {
            let dist = point.m_distance(self);
            if dist == closest_dist {
                closest_point_count += 1;
            }
            if dist < closest_dist {
                closest_point_count = 1;
                closest_dist = dist;
                closest_point = Point::new(point.x,point.y);
            }        
        }

        if closest_point_count == 1 {
            Some(closest_point)
        }
        else{
            None
        }
    }

    pub fn find_total_distance(&self, points: &HashSet<Point>) -> usize {
        let mut total_distance = 0;
        for point in points {
            total_distance += point.m_distance(self);
        }

        total_distance
    }
}