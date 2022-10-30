use crate::champions::CHAMPIONS;
use rand::seq::SliceRandom;

pub fn get_random_champions_from_seq(
    amount: usize,
    champions: &'static [&'static str],
) -> Vec<&'static str> {
    let mut rng = &mut rand::thread_rng();

    champions
        .choose_multiple(&mut rng, amount)
        .cloned()
        .collect::<Vec<&str>>()
}

pub fn get_random_champions(amount: usize) -> Vec<&'static str> {
    get_random_champions_from_seq(amount, CHAMPIONS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_random_champions() {
        let mut all_champions = CHAMPIONS.to_vec();
        all_champions.sort();

        let mut all_random_champions = get_random_champions_from_seq(CHAMPIONS.len(), CHAMPIONS);
        all_random_champions.sort();

        assert_eq!(all_champions, all_random_champions);
    }

    #[test]
    fn test123() {
        assert_eq!(1, 1);
    }
}
