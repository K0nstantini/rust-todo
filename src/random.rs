use rand::Rng;

pub struct RandomItem {
    pub weight: u32,
    pub time: u32,
}

pub fn get(items: Vec<RandomItem>) -> usize {
    let mut rng = rand::thread_rng();
    let max_range = items.iter().map(|c| c.weight).sum();
    let mut rand_weight = rng.gen_range(1..=max_range);

    for (i, item) in items.iter().enumerate() {
        if rand_weight <= item.weight {
            return i;
        }
        rand_weight -= item.weight;
    }
    unreachable!();
}