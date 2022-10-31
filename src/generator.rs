use rand::seq::SliceRandom;

pub fn get_random_champions_from_pool<'a>(
    amount: usize,
    champions: &[&'a str],
) -> Vec<&'a str> {
    let mut rng = &mut rand::thread_rng();

    champions
        .choose_multiple(&mut rng, amount)
        .cloned()
        .collect::<Vec<&str>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::champions::CHAMPIONS;

    #[test]
    fn all_distinct_champions() {
        let mut all_champions = CHAMPIONS.to_vec();
        all_champions.sort();

        let mut all_random_champions = get_random_champions_from_pool(CHAMPIONS.len(), CHAMPIONS);
        all_random_champions.sort();

        assert_eq!(all_champions, all_random_champions);
    }

    #[test]
    fn custom_distinct_champions() {
        let mut champion_pool = vec![ "Champion1", "Champion4", "Champion3", "Champion2", "Champion5" ];

        let mut random_champions = get_random_champions_from_pool(champion_pool.len(), &champion_pool);

        champion_pool.sort();
        random_champions.sort();

        assert_eq!(champion_pool, random_champions);
    }
}
