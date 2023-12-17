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
        let (hand, bid) = value
            .split_once(' ')
            .unwrap();
        
        let bid: u64 = bid.parse::<u64>().unwrap();

        let cards: Vec<u64> = hand
            .chars()
            .map(|chr| {
                match chr {
                    'J' => 1, '2' => 2, '3' => 3, '4' => 4,
                    '5' => 5, '6' => 6, '7' => 7, '8' => 8,
                    '9' => 9, 'T' => 10,'Q' => 11,'K' => 12,
                    'A' => 13, _ => unreachable!(),
                }
            })
            .collect();
        
        // time to calculate the score, which is split into 2 parts:
        // * the main part, which is derived from how many similar cards the player has
        // * the secondary part, derived from the power of the cards in order
        let mut main_score_precursor: Vec<u64> = (2_u64..=13_u64)
            .into_iter()
            .map(|val| {
                cards.iter().filter(|card_value| {**card_value == val}).count() as u64
            })
            .collect();

        let main_score_joker_boost: u64 = cards.iter().filter(|card_value| {**card_value == 1}).count() as u64;
        
        main_score_precursor.sort_by(|a, b| {b.cmp(a)});
        main_score_precursor[0] += main_score_joker_boost;

        let main_score: u64 = main_score_precursor
            .into_iter()
            .map(|precursor| {
                if precursor > 1 {
                    (1..=precursor).product::<u64>()
                } else {
                    0
                }
            })
            .fold(0, |acc, value| {acc + value});

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
    fn run(input: &str) -> u64 {
        use crate::part2::Hand;

        let mut hands: Vec<Hand> = Vec::new();
        for line in input.trim().lines() {
            Hand::from(line).incorporate(&mut hands);
        }
        hands
            .into_iter()
            .enumerate()
            .fold(0, |acc, (idx, hand)| {acc + hand.bid * (idx as u64+1)})
    }
    #[test]
    fn primary() {
        let input: &str = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
        
        assert_eq!(
            run(input),
            5905
        )
    }
    #[test]
    fn secondary() {
        let input: &str = "JAAKK 1\nJJJAK 2";

        assert_eq!(
            run(input),
            5
        )
    }
    #[test]
    fn tertiary() {
        let input: &str = "JJJJJ 1\nAJJJJ 2";

        assert_eq!(
            run(input),
            5
        )
    }
    #[test]
    fn quarternary() {
        let input: &str = "2345A 1\nQ2KJJ 13\nQ2Q2Q 19\nT3T3J 17\nT3Q33 11\n2345J 3\nJ345A 2\n32T3K 5\nT55J5 29\nKK677 7\nKTJJT 34\nQQQJA 31\nJJJJJ 37\nJAAAA 43\nAAAAJ 59\nAAAAA 61\n2AAAA 23\n2JJJJ 53\nJJJJ2 41";

        assert_eq!(
            run(input),
            6839,
        )
    }
}