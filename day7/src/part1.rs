#[derive(Debug)]
pub struct Hand {
    pub score: u64,
    pub bid: u64,
}

impl Hand {
    pub fn incorporate(self, others: &mut Vec<Self>) {
        let mut target_index: usize = 0;
        for (other_idx, other) in others.iter().enumerate() {
            if self.score < other.score {
                target_index = other_idx;
                break;
            }
            target_index += 1
        }
        others.insert(target_index, self);
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let value: Vec<&str> = value
            .split_whitespace()
            .collect();
        
        let bid: u64 = value[1].parse::<u64>().unwrap();

        let cards: Vec<u64> = value[0]
            .chars()
            .map(|chr| {
                match chr {
                    '2' => 2, '3' => 3, '4' => 4, '5' => 5,
                    '6' => 6, '7' => 7, '8' => 8, '9' => 9,
                    'T' => 10,'J' => 11,'Q' => 12,'K' => 13,
                    'A' => 14, _ => unreachable!(),
                }
            })
            .collect();
        
        // time to calculate the score, which is split into 2 parts:
        // * the main part, which is derived from how many similar cards the player has
        // * the secondary part, derived from the power of the cards in order
        let main_score: u64 = (2_u64..=14_u64)
            .into_iter()
            .map(|val| {
                // get the factorial of how many
                let amount: usize = cards.iter().filter(|card_value| {**card_value == val}).count();
                if amount > 1 {
                    (1..=amount).product::<usize>() as u64
                } else {
                    0
                }
            })
            .fold(0, |acc, x| {acc + x});

        let secondary_score: u64 = cards
            .iter()
            .rev()
            .enumerate()
            .map(|(index, value)| {*value * u64::pow(100, index as u32)})
            .fold(0, |acc, x| {acc + x});

        let score: u64 = main_score * 10000000000 + secondary_score;

        Self { score, bid }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn primairy() {
        use crate::part1::Hand;

        let input: &str = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
        let mut hands: Vec<Hand> = Vec::new();
        for line in input.trim().lines() {
            Hand::from(line).incorporate(&mut hands);
        }

        let winnings: u64 = hands
            .into_iter()
            .enumerate()
            .fold(0, |acc, (idx, hand)| {acc + hand.bid * (idx as u64+1)});

        assert_eq!(
            winnings,
            6440
        )
    }
}