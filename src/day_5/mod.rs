#[derive(Debug)]
pub struct Mapping {
    pub in_start: i32,
    pub out_start: i32,
    pub range: i32,    
}

impl Mapping {
    fn map(&self, input: i32) -> i32 {
        let offset = input - self.in_start;
        self.out_start + offset
    }

    fn in_range(&self, input: i32) -> bool {
        input >= self.in_start && input <= self.in_start + self.range
    }    
}

#[derive(Debug)]
pub struct Ranges {
    pub ranges: Vec<Mapping>,
}

impl Ranges {
    pub fn map(&self, input: i32) -> i32 {
        for mapping in self.ranges.iter() {
            if mapping.in_range(input) {
                return mapping.map(input);
            }
        }

        input
    }
}


        
