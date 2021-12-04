use std::str::from_utf8;

#[derive(Default, Debug)]
pub struct BingoCard {
    card: [[u8; 5]; 5],
    mark: [[bool; 5]; 5],
}

impl BingoCard {
    pub fn new(input: &[u8; 75]) -> BingoCard {
        let mut card = [[0u8; 5]; 5];
        for i in 0..5 {
            for j in 0..5 {
                let offset = (i * 5 + j) * 3;
                let n = from_utf8(&input[offset..offset + 2]).unwrap().trim_start();
                let n = n.parse::<u8>().unwrap();
                card[i][j] = n;
            }
        }
        BingoCard {
            card,
            ..Default::default()
        }
    }

    pub fn mark(&mut self, draw: u8) -> Option<i32> {
        for i in 0..5 {
            for j in 0..5 {
                if self.card[i][j] == draw {
                    self.mark[i][j] = true;
                    return self.check_bingo().map(|sum| sum * draw as i32);
                }
            }
        }
        None
    }

    fn check_bingo(&self) -> Option<i32> {
        for i in 0..5 {
            if (self.mark[i][0]
                && self.mark[i][1]
                && self.mark[i][2]
                && self.mark[i][3]
                && self.mark[i][4])
                || (self.mark[0][i]
                    && self.mark[1][i]
                    && self.mark[2][i]
                    && self.mark[3][i]
                    && self.mark[4][i])
            {
                return Some(self.sum_unmarked());
            }
        }
        None
    }

    fn sum_unmarked(&self) -> i32 {
        let mut sum: i32 = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.mark[i][j] {
                    sum += self.card[i][j] as i32
                }
            }
        }
        sum
    }
}
