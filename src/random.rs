use std::ops::Range;

use rand::Rng;

const OFFSET: f64 = 0.15;

#[derive(Debug)]
pub struct RandomData {
    pub weight: u32,
    pub time: u32,
}

#[derive(Debug)]
struct Rates {
    percent: f64,
    rate: f64,
}

impl Rates {
    fn new(percent: f64, rate: f64) -> Self {
        let rate = if rate == 0.0 { 0.0001 } else { rate }; // fix incorrect calc weight if rate == 0
        Rates { percent, rate }
    }

    fn from_data(items: &[RandomData]) -> Vec<Self> {
        let (weights, times) = items
            .iter()
            .fold((0.0, 0.0), |acc, it|
                (acc.0 + it.weight as f64, acc.1 + it.time as f64),
            );

        let rate = move |it: &RandomData| {
            let (weight, time) = (it.weight as f64, it.time as f64);
            (time * weights) / (times * weight)
        };

        let percent = |it: &RandomData| it.weight as f64 / weights;

        items
            .iter()
            .map(|it| Self::new(percent(it), rate(it)))
            .collect()
    }
}

fn winner_condition() -> fn(&Rates) -> bool {
    |it: &Rates| it.rate > 1.0
}

fn loser_condition() -> fn(&Rates) -> bool {
    |it: &Rates| it.rate < 1.0
}

fn filter_sum(
    items: &[Rates],
    condition: impl Fn(&Rates) -> bool,
    field: impl Fn(&Rates) -> f64,
) -> f64 {
    items
        .iter()
        .filter_map(|it|
            if condition(it) {
                Some(field(it))
            } else {
                None
            })
        .sum()
}

fn get_offset(items: &[Rates]) -> f64 {
    let sum = |condition| filter_sum(items, condition, |it| it.percent);
    let winners_sum = sum(winner_condition());
    let losers_sum = sum(loser_condition());

    OFFSET
        .min(winners_sum)
        .min(1.0 - losers_sum)
}

fn calc_weight(items: &[Rates]) -> impl Fn(&Rates) -> u32 {
    let winners_rates_sum = filter_sum(items, winner_condition(), |it| it.rate);
    let loser_rates_sum = filter_sum(items, loser_condition(), |it| 1.0 / it.rate);

    let offset = get_offset(items);
    let winner_offset = move |it: &Rates| (offset / winners_rates_sum) * it.rate;
    let loser_offset = move |it: &Rates| (offset / loser_rates_sum) / it.rate;

    move |it| {
        let offset = match it {
            _ if winner_condition()(it) => -winner_offset(it),
            _ if loser_condition()(it) => loser_offset(it),
            _ => 0.0
        };
        (((it.percent + offset) * 100.0).round() as u32)
            .max(1) // leave ~1% chance
    }
}

fn update_weights(items: &[RandomData]) -> Vec<u32> {
    let no_times = items.iter().all(|i| i.time == 0);
    if no_times {
        items.iter().map(|i| i.weight).collect()
    } else {
        let rates = Rates::from_data(items);
        // rates.iter().for_each(|it| println!("{:?}", it));
        // println!();
        let calc_weights = calc_weight(&rates);
        rates
            .iter()
            .map(calc_weights)
            .collect()
    }
}

pub fn get(items: &[RandomData]) -> usize {
    let weights = update_weights(items);
    let all_weights = weights.iter().sum();

    let mut rng = rand::thread_rng();
    let mut rand_weight = rng.gen_range(1..=all_weights);
    show_weights(&weights, rand_weight);

    for (i, weight) in weights.into_iter().enumerate() {
        if rand_weight <= weight {
            return i;
        }
        rand_weight -= weight;
    }
    unreachable!();
}

fn show_weights(weights: &[u32], rand: u32) {
    print!("{rand} from ");
    weights.iter().for_each(|it| print!("{it} "));
    println!();
}

#[test]
fn update_weights_test() {
    let item = |weight, time| RandomData { weight, time };

    let items = [
        item(1, 100),
        item(1, 80),
        item(1, 50),
        item(3, 10),
        item(2, 6),
        item(2, 4),
    ];
    let weights = update_weights(&items);
    assert_eq!(weights, [6, 7, 8, 33, 23, 24]);

    let items = [
        item(3, 1080),
        item(1, 0),
    ];
    let weights = update_weights(&items);
    assert_eq!(weights, [65, 35]);

    let items = [
        item(6, 100),
        item(2, 0),
        item(2, 0),
    ];
    let weights = update_weights(&items);
    assert_eq!(weights, [50, 25, 25]);
}

#[test]
#[ignore]
fn random_test() {
    let count_tasks = 1000;
    let time_range = 15..60;
    let weights = [1, 1];

    let items = run_sim(count_tasks, time_range, weights);
    let overall_time: u32 = items
        .iter()
        .map(|i| i.time)
        .sum();

    for (i, item) in items.iter().enumerate() {
        println!("Task{}: {:.0}", i, item.time as f64 / overall_time as f64 * 100.0);
    }

    assert!(true);
}

#[allow(dead_code)]
fn run_sim(tasks: i32, time_range: Range<u32>, weights: [u32; 2]) -> Vec<RandomData> {
    let mut items: Vec<_> = weights
        .iter()
        .map(|&weight| RandomData { weight, time: 0 })
        .collect();

    let mut rng = rand::thread_rng();
    for _ in 0..tasks {
        let idx = get(&items);
        let time = rng.gen_range(time_range.clone());
        items.get_mut(idx).unwrap().time = time;
    }
    items
}