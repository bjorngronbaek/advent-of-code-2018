use std::fmt;

pub struct Claim {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub length: u32,
    pub height: u32
}

impl Claim{
    //#1 @ 1,3: 4x4
    pub fn new(id: u32, x:u32, y:u32, length:u32, height: u32) -> Claim {
        Claim {
            id:id,
            x:x,
            y:y,
            length:length,
            height:height
        }
    }

    pub fn overlaps(self, claim:&Claim) -> bool {
        let tl_1 = (self.x,self.y);
        let br_1 = (self.x+self.length,self.y+self.height);

        let tl_2 = (claim.x,claim.y);
        let br_2 = (claim.x+claim.length,claim.y+claim.height);

        if tl_1.0 > br_2.0 || tl_2.0 > br_1.0 {
            return false;
        }

        if tl_1.1 < br_2.1 || tl_2.1 > br_1.1 {
            return false;
        }

        return true;
    }
}

pub struct Piece {
    width: u32,
    height: u32,
    pub claim_count: Vec<u32>
}

impl Piece {
    pub fn new(width :u32, height :u32) -> Piece {
        let claims = (0..width * height).map(|_x| 0).collect();
        Piece {
            width: width,
            height: height,
            claim_count: claims
        }
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn claim(&mut self, claim: &Claim) {
        for dy in 0..claim.height {
            for dx in 0..claim.length {
                let xpos = claim.x + dx;
                let ypos = claim.y + dy;
                //println!("Updating {}x{}",xpos,ypos);
                let index = self.get_index(ypos,xpos);
                self.claim_count[index] = self.claim_count[index] + 1;
            }
        }
    }

    pub fn unique_claim(&self, claim: &Claim) -> bool {
        //println!("Investigating id: {}",claim);
        let mut isunique = true;
        for dy in 0..claim.height {
            for dx in 0..claim.length {
                let xpos = claim.x + dx;
                let ypos = claim.y + dy;

                let count = self.claim_count[self.get_index(ypos,xpos)];
                //println!("pos: ({},{}), value: {}",xpos,ypos,count);
                if count > 1 {
                    isunique = false;
                }
            }
        }

        isunique
    }
}

impl fmt::Display for Claim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"#{} @ {},{}: {}x{}",self.id,self.x,self.y,self.length,self.height);
        Ok(())
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.claim_count.as_slice().chunks(self.width as usize) {
            for &claim_count in line {
                write!(f, "{}", claim_count)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
