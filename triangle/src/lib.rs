pub struct Triangle {
    sides: [u64; 3]
}

/*
    equilateral = all three sides the same length.
    isosceles = has at least two sides the same length. (It is sometimes specified as having exactly two sides the same length, but for the purposes of this exercise we'll say at least two.)
    scalene = has all sides of different lengths.
*/

/*
    the triangle inequality states that
    z ≤ x + y 
    
    For a shape to be a triangle at all,
    all sides have to be of length > 0,
    and the sum of the lengths of any two sides must be greater than 
    or equal to the length of the third side.
*/

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if Triangle::inequality(sides){
            Some(Triangle{sides: sides})
        }else{
            None
        }  
    }

    fn inequality(sides: [u64; 3]) -> bool {
        Triangle::sum_of_length(sides) && Triangle::all_sides_no_zero(sides)
    }

    fn sum_of_length(sides: [u64; 3]) -> bool{
        sides[0] + sides[1] >= sides[2] && sides[0] + sides[2] >= sides[1] && sides[1] + sides[2] >= sides[0]
    }

    fn all_sides_no_zero (sides: [u64; 3]) ->bool{
        let mut count = 0;
        for i in &sides{
            if i > &0{
                count += 1;
            }
        }
        return count == 3;
    }

    fn equal_sides(&self) -> u64{
        let mut count = 0;
        for x in 0..(self.sides).len() {
            for y in x+1..(self.sides).len(){
                if self.sides[x] == self.sides[y]{
                    count+=1;
                }
            }
        }
        count
    }

    pub fn is_equilateral(&self) -> bool {
        Triangle::equal_sides(self) == 3
    }

    pub fn is_scalene(&self) -> bool {
        Triangle::equal_sides(self) == 0
    }

    pub fn is_isosceles(&self) -> bool {
        Triangle::equal_sides(self) >= 1
    }
}
