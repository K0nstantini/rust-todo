use std::cmp::min;
use std::ops::Range;

use rand::Rng;

const OFFSET: f64 = 0.1;
const MAX_PERCENT: f64 = 0.6;

// type Tii =  Fn(&RandomData) -> 32;

pub struct RandomData {
    pub weight: u32,
    pub time: u32,
}

struct Rates {
    percent: f64,
    rate: f64,
}

impl Rates {
    fn new(percent: f64, rate: f64) -> Self {
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

// struct RandomItem {
//     id: usize,
//     percent_need: f64,
//     percent_done: f64,
//     rate: f64,
//     percent_next: f64,
// }
//
// impl RandomItem {
//     fn new(id: usize, item: &RandomData, weights: u32, times: u32) -> Self {
//         let percent_need = item.weight as f64 / weights as f64;
//         let percent_done = item.time as f64 / times as f64;
//         let time_need = times as f64 * percent_need;
//         let rate = item.time as f64 / time_need;
//         RandomItem {
//             id,
//             percent_need,
//             percent_done,
//             rate,
//             percent_next: 0.0,
//         }
//     }
// }
//
// struct RandomModel {
//     items: Vec<RandomItem>,
// }
//
// impl RandomModel {
//     fn new(items: &[RandomData]) -> Self {
//         let weights: u32 = items.iter().map(|i| i.weight).sum();
//         let times: u32 = items.iter().map(|i| i.time).sum();
//         let items = items
//             .iter()
//             .enumerate()
//             .map(|(i, it)| RandomItem::new(i, it, weights, times))
//             .collect();
//         RandomModel { items }
//     }
//
//     fn calc(&mut self) {
//         let winners = self.winners();
//         let losers = self.losers();
//         self.check_items(&losers, &winners);
//
//         let top_winner = self.top_winner();
//         let top_loser = self.top_loser();
//
//         let offset = Self::offset(&winners, &losers);
//         let top_win_offset = Self::offset_top_winner(&winners, top_winner, offset);
//         let top_lose_offset = Self::offset_top_loser(&losers, top_loser, offset);
//     }
//
//     fn offset_top_winner(items: &[&RandomItem], item: &RandomItem, offset: f64) -> f64 {
//         let rates: f64 = items
//             .iter()
//             .map(|it| it.rate)
//             .sum();
//         offset * item.rate / rates
//     }
//
//     fn offset_top_loser(items: &[&RandomItem], item: &RandomItem, offset: f64) -> f64 {
//         let rates: f64 = items
//             .iter()
//             .map(|it| 1.0 / it.rate)
//             .sum();
//         offset / (item.rate * rates)
//     }
//
//     fn offset(winners: &[&RandomItem], losers: &[&RandomItem]) -> f64 {
//         let sum = |items: &[&RandomItem]|
//             items
//                 .iter()
//                 .map(|it| it.percent_need)
//                 .sum::<f64>();
//
//         OFFSET
//             .min(sum(winners))
//             .min(1.0 - sum(losers))
//     }
//
//     fn check_items(&self, losers: &[&RandomItem], winners: &[&RandomItem]) {
//         let message = match (losers.len(), winners.len()) {
//             (0, _) => "Not found losers",
//             (_, 0) => "Not found winners",
//             (l, w) if l + w != self.items.len() => "Invalid count of losers and winners",
//             _ => return
//         };
//         panic!("{}", message);
//     }
//
//     fn losers(&self) -> Vec<&RandomItem> {
//         self.filter(|it| it.percent_done < it.percent_need)
//     }
//
//     fn winners(&self) -> Vec<&RandomItem> {
//         self.filter(|it| it.percent_done > it.percent_need)
//     }
//
//     fn top_winner(&self) -> &RandomItem {
//         self.items
//             .iter()
//             .max_by(|a, b| a.rate.total_cmp(&b.rate))
//             .unwrap()
//     }
//
//     fn top_loser(&self) -> &RandomItem {
//         self.items
//             .iter()
//             .min_by(|a, b| a.rate.total_cmp(&b.rate))
//             .unwrap()
//     }
//
//     fn filter<F: FnMut(&RandomItem) -> bool>(&self, mut filter: F) -> Vec<&RandomItem> {
//         self.items
//             .iter()
//             .filter(|it| filter(it))
//             .collect()
//     }
// }
//
// fn rate_fn(items: &[RandomData]) -> impl Fn(&RandomData) -> f64 {
//     let weights: f64 = items.iter().map(|i| i.weight as f64).sum();
//     let times: f64 = items.iter().map(|i| i.time as f64).sum();
//
//     move |it| {
//         let (weight, time) = (it.weight as f64, it.time as f64);
//         (time * weights) / (times * weight)
//     }
// }

// fn winner_weight_fn<R: Fn(&RandomData) -> f64>(items: &[RandomData], rate: R, offset: f64) -> impl Fn(&RandomData) -> u32 {
//     let rates: f64 = items
//         .iter()
//         .filter_map(|it| match rate(it) {
//             rate if rate > 1.0 => Some(rate),
//             _ => None
//         })
//         .sum();
//
//     move |it| {
//         let offset = offset * rate(it) / rates;
//         it.weight - offset.round() as u32
//     }
// }
//
//
// fn loser_weight_fn<R: Fn(&RandomData) -> f64>(items: &[RandomData], rate: R, offset: f64) -> impl Fn(&RandomData) -> u32 {
//     let rates: f64 = items
//         .iter()
//         .filter_map(|it| match rate(it) {
//             rate if rate < 1.0 => Some(1.0 / rate),
//             _ => None
//         })
//         .sum();
//
//     move |it| {
//         let offset = offset / (rate(it) * rates);
//         it.weight - offset.round() as u32
//     }
// }

// fn total_offset(items: &[RandomData]) -> f64 {
//     let sum = |items: &[&RandomData]|
//         items
//             .iter()
//             .map(|it| it.percent_need)
//             .sum::<f64>();
//
//     OFFSET
//         .min(sum(winners))
//         .min(1.0 - sum(losers))
// }
//
// fn winner_offset(offset: f64) -> impl Fn(f64, f64) -> f64 {
//     move |rate, rates| offset * rate / rates
// }
//
//
// fn loser_offset(offset: f64) -> impl Fn(f64, f64) -> f64 {
//     move |rate, rates| offset / (rate * rates)
// }
//
// fn winner_rate_condition(rate: impl Fn(&RandomData) -> f64) -> impl Fn(&RandomData) -> Option<f64> {
//     move |it| match rate(it) {
//         rate if rate > 1.0 => Some(rate),
//         _ => None
//     }
// }
//
// fn loser_rate_condition(rate: impl Fn(&RandomData) -> f64) -> impl Fn(&RandomData) -> Option<f64> {
//     move |it| match rate(it) {
//         rate if rate < 1.0 => Some(1.0 / rate),
//         _ => None
//     }
// }
//
// fn weight_fn(
//     items: &[RandomData],
//     rates_condition: impl Fn(&RandomData) -> Option<f64>,
//     offset: impl Fn(f64, f64) -> f64,
// ) -> impl Fn(&RandomData, f64) -> u32 {
//     let rates: f64 = items
//         .iter()
//         .filter_map(rates_condition)
//         .sum();
//
//     move |it, rate| {
//         let offset = offset(rate, rates);
//         it.weight - offset.round() as u32
//     }
// }
//
// fn calc_weights_fn(items: &[RandomData], rate: impl Fn(&RandomData) -> f64) -> impl Fn(&RandomData) -> u32 {
//     // let rate_fn = rate_fn(items);
//     let offset = total_offset(items);
//     let winner = weight_fn(items, winner_rate_condition(&rate), winner_offset(offset));
//     let loser = weight_fn(items, loser_rate_condition(&rate), loser_offset(offset));
//
//
//     move |it| match &rate(it) {
//         &rate if rate > 1.0 => winner(it, rate),
//         &rate if rate < 1.0 => loser(it, rate),
//         _ => it.weight
//     }
// }

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
            _ if winner_condition()(it) => - winner_offset(it),
            _ if loser_condition()(it) => loser_offset(it),
            _ => 0.0
        };
        ((it.percent + offset) * 100.0).round() as u32
    }
}

fn update_weights(items: &[RandomData]) -> Vec<u32> {
    let no_times = items.iter().all(|i| i.time == 0);
    if no_times {
        items.iter().map(|i| i.weight).collect()
    } else {
        let rates = Rates::from_data(items);
        rates.iter().for_each(|it| println!("Percent: {}, Rate: {}", it.percent, it.rate));
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

    for (i, item) in items.iter().enumerate() {
        if rand_weight <= item.weight {
            return i;
        }
        rand_weight -= item.weight;
    }
    unreachable!();
}


pub fn get_old(items: &[RandomData]) -> usize {
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
    update_weights(&items)
        .iter()
        .for_each(|w| println!("{}", w));

    assert!(true);
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