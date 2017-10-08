pub struct PascalsTriangle {
    row: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row: row_count }
    }

    fn caculate(&self, resp_vec: &mut Vec<Vec<u32>>) {
        for row in 3..self.row + 1 {
            let mut item_vec = vec![1; row as usize];
            for i in (1..row - 1) {
                item_vec[i as usize] = resp_vec[(row - 2) as usize][(i - 1) as usize] + resp_vec[(row - 2) as usize][i as usize];
            }
            resp_vec.push(item_vec);
        }
    }

    pub fn rows_two(&self) -> Vec<Vec<u32>> {
        let mut resp: Vec<Vec<u32>> = Vec::new();
        match self.row {
            0 => resp,
            1 => { resp.push(vec![1]); resp},
            2 => { resp.push(vec![1]); resp.push(vec![1, 1]); resp},
            _ => {
                resp.push(vec![1]);
                resp.push(vec![1, 1]);
                self.caculate(&mut resp);
                resp
            }
        }
    }

    pub fn row(&self, row: u32) -> Vec<u32> {
        (1..row + 1).fold(vec![1], |mut acc, index| {
            if let Some(&last) = acc.last() {
                acc.push(last * (row + 1 - index) / index);
            }
            acc
        })
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.row).map(|row| self.row(row)).collect()
    }
}
