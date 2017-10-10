pub type TriangleResult<T> = Result<T, ()>;

pub struct Triangle {
    sides: [usize; 3],
}

impl Triangle {
    pub fn build(mut sides: [usize; 3]) -> TriangleResult<Triangle> {
        sides.sort();
        match sides[0] + sides[1] > sides[2] && sides[2] - sides[1] < sides[0] {
            true => Ok(Triangle { sides: sides }),
            false => Err(())
        }
    }

    pub fn is_equilateral(&self) -> bool {
        println!("{:?}", self.sides);
        self.sides[0] == self.sides[1] && self.sides[0] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        self.sides[0] != self.sides[1] && self.sides[1] != self.sides[2]
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1] || self.sides[1] == self.sides[2]
    }
}
