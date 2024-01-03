use std::collections::BTreeSet;

use itertools::Itertools;
use rand::Rng;
use trace::trace;

trace::init_depth_var!();

use crate::Solver;

pub struct Solution;
impl Solver<i64, i64> for Solution {
    fn solve(&self, input: &str) -> (i64, i64) {
        solve(input)
    }
}

type RangeInt = i64;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
struct SeedRange {
    min: RangeInt,
    max: RangeInt,
    len: RangeInt,
}

#[derive(Debug, Clone)]
struct RangeMapping {
    src_min: RangeInt,
    src_max: RangeInt,
    dst_min: RangeInt,
    dst_max: RangeInt,
    //   len: RangeInt,
}

// #[trace]
fn print_interval(min: RangeInt, max: RangeInt, c: char) {
    assert!(min <= max);
    print!("[{:3}-{:3}] ", min, max);
    for _ in 0..min {
        print!(".");
    }
    for _ in min..=max {
        print!("{c}");
    }
    for _ in max + 1..=200 {
        print!(".");
    }
    println!();
}

// #[trace(disable(new_random))]
impl SeedRange {
    fn new_singleton(n: RangeInt) -> Self {
        Self {
            min: n,
            max: n,
            len: 1,
        }
    }

    fn new(min: RangeInt, max: RangeInt) -> Self {
        assert!(min <= max);
        Self {
            min,
            max,
            len: max - min + 1,
        }
    }

    fn check(&self) {
        assert!(self.min > 0);
        assert!(self.min <= self.max);
        assert!(self.max - self.min + 1 == self.len);
    }

    fn new_random<R: Rng>(rng: &mut R) -> Self {
        let min = rng.gen_range(0..80);
        let len = rng.gen_range(1..40);
        Self {
            min,
            max: min + len - 1,
            len,
        }
    }

    fn print(&self, c: char) {
        println!("Seed range:");
        print_interval(self.min, self.max, c);
    }

    //#[trace]
    fn apply_mapping(&self, mapping: &RangeMapping) -> Option<Vec<SeedRange>> {
        if self.max < mapping.src_min || self.min > mapping.src_max {
            // println!("[apply-mapping] {:?} rule not applicable", self);
            None
        } else if self.min < mapping.src_min
            && self.max >= mapping.src_min
            && self.max <= mapping.src_max
        {
            // mapping-src:          #######
            // seed range:       #########
            // mapping-dest:                      v
            // result:           ####             #####
            // println!(
            //     "[apply-mapping] {:?} seed range overlaps lower half of mapping range",
            //     self
            // );
            let lower_part = Self::new(self.min, mapping.src_min - 1);
            let upper_part = Self::new(
                mapping.dst_min,
                mapping.dst_min + (self.len - lower_part.len) - 1,
            );
            Some(vec![lower_part, upper_part])
        } else if self.min >= mapping.src_min && self.max <= mapping.src_max {
            // mapping-src:      ###########
            // seed range:       .########..
            // mapping-dest:                      v
            // result:                            .########

            let offset = self.min - mapping.src_min;
            assert!(offset >= 0);
            // println!(
            //     "[apply-mapping] {:?} seed range is enclosed by mapping range",
            //     self
            // );
            Some(vec![Self::new(
                mapping.dst_min + offset,
                mapping.dst_min + offset + self.len - 1,
            )])
        } else if self.min < mapping.src_min && self.max > mapping.src_max {
            // mapping-src:         ####
            // seed range:        ########
            // mapping-dest:                      v
            // result:            ##    ##        ####
            // println!(
            //     "[apply-mapping] {:?} mapping range is enclosed by seed range",
            //     self
            // );
            Some(vec![
                Self::new(self.min, mapping.src_min - 1),
                Self::new(mapping.dst_min, mapping.dst_max),
                Self::new(mapping.src_max + 1, self.max),
            ])
        } else if self.min <= mapping.src_max && self.max > mapping.src_max {
            // mapping-src:       ########
            // seed range:        ....########
            // mapping-dest:                      v
            // result:                    ####    ....####
            // println!(
            //     "[apply-mapping] {:?} seed range overlaps upper half of mapping range",
            //     self
            // );
            // seed range overlaps with upper bound of the mapping range
            let offset = self.min - mapping.src_min;
            assert!(offset >= 0);
            Some(vec![
                Self::new(mapping.src_max + 1, self.max),
                Self::new(mapping.dst_min + offset, mapping.dst_max),
            ])
        } else {
            unreachable!()
        }
    }
}

// #[trace(disable(new_random))]
impl RangeMapping {
    fn new(src_min: RangeInt, src_max: RangeInt, dst_min: RangeInt, dst_max: RangeInt) -> Self {
        assert_eq!(src_max - src_min, dst_max - dst_min);
        Self {
            src_min,
            src_max,
            dst_min,
            dst_max,
        }
    }

    // #[trace]
    fn new_from_line(input: &str) -> Self {
        let (dst, src, len) = input.split(' ').collect_tuple().unwrap();
        let len: RangeInt = len.parse().unwrap();
        let src_min = src.parse().unwrap();
        let src_max = src_min + len - 1;
        let dst_min = dst.parse().unwrap();
        let dst_max = dst_min + len - 1;
        Self::new(src_min, src_max, dst_min, dst_max)
    }

    fn new_random<R: Rng>(rng: &mut R) -> Self {
        let src_min = rng.gen_range(0..80);
        let dst_min = rng.gen_range(100..120);
        let len = rng.gen_range(1..30);
        Self {
            src_min,
            src_max: src_min + len - 1,
            dst_min,
            dst_max: dst_min + len - 1,
        }
    }

    fn print(&self) {
        println!("Mapping, source range:");
        print_interval(self.src_min, self.src_max, 'V');
        println!("Destination range:");
        print_interval(self.dst_min, self.dst_max, 'T');
    }
}

fn solve(input: &str) -> (i64, i64) {
    let p1 = solve_p1(input);
    let p2 = solve_p2(input);
    (p1, p2)
}

fn solve_p1(input: &str) -> i64 {
    let elems = input.split("\n\n").collect_vec();
    let seeds = parse_seeds_p1(elems[0]);
    let maps = parse_maps(&elems[1..]);

    apply_maps(&seeds, &maps)
}

fn solve_p2(input: &str) -> i64 {
    let elems = input.split("\n\n").collect_vec();
    let seeds = parse_seeds_p2(elems[0]);
    let maps = parse_maps(&elems[1..]);

    apply_maps(&seeds, &maps)
}

fn apply_maps(seeds: &[SeedRange], maps: &Vec<Vec<RangeMapping>>) -> i64 {
    let mut min_location = i64::MAX;

    for seed_range in seeds {
        // println!("\n\n=== Processing seed range");
        // seed_range.print('S');

        // "seeds_in" is the seeds as they are being passed through the mapping
        // ranges.
        let mut seeds_in: BTreeSet<SeedRange> = BTreeSet::new();
        let mut seeds_out: BTreeSet<SeedRange> = BTreeSet::new();

        seeds_in.insert(*seed_range);

        // Filter each seed range through all of the maps. Each map
        // consists of a number of mapping-ranges.
        for map in maps {
            // println!("\n---- Applying map {:?}", map[0]);

            // Apply each mapping range to the seed range.
            for seed in &seeds_in {
                let mut is_applicable = false;

                for range_mapping in map {
                    if let Some(applied_mapping) = seed.apply_mapping(range_mapping) {
                        for m in &applied_mapping {
                            m.check();
                            seeds_out.insert(*m);
                            is_applicable = true;
                        }

                        assert_eq!(applied_mapping.iter().map(|m| m.len).sum::<i64>(), seed.len);
                        break;
                    }
                }

                if !is_applicable {
                    seeds_out.insert(*seed);
                }
            }

            // println!("Seed ranges after applying map: {:?}", seeds_out);

            seeds_in.clone_from(&seeds_out);
            seeds_out.clear();
        }

        // println!("{:?}\n->\n{:?}", seed_range, seeds_in);
        min_location = min_location.min(seeds_in.first().unwrap().min);
    }

    min_location
}

fn parse_seeds_p1(input: &str) -> Vec<SeedRange> {
    input
        .split(' ')
        .skip(1)
        .filter_map(|s| s.parse().ok())
        .map(SeedRange::new_singleton)
        .collect()
}

fn parse_seeds_p2(input: &str) -> Vec<SeedRange> {
    input
        .split(' ')
        .skip(1)
        .filter_map(|s| s.parse().ok())
        .chunks(2)
        .into_iter()
        .map(|chunk| {
            let (start, len) = chunk.collect_tuple().unwrap();
            SeedRange::new(start, start + len - 1)
        })
        .collect()
}

fn parse_maps(elems: &[&str]) -> Vec<Vec<RangeMapping>> {
    elems
        .iter()
        .map(|s| {
            s.split('\n')
                .skip(1)
                .filter(|s| !s.is_empty())
                .map(RangeMapping::new_from_line)
                .collect_vec()
        })
        .collect_vec()
}

#[cfg(test)]

mod tests {

    // use rand::SeedableRng;
    // use rand_chacha::ChaCha12Rng;

    use super::*;

    // #[test]
    // fn test_apply_mapping() {
    //     let mapping = RangeMapping::new_from_line("50 98 2");
    //     assert_eq!(
    //         vec![
    //             SeedRange::new_singleton(97),
    //             SeedRange::new_from_min_max(50, 51)
    //         ],
    //         SeedRange::new_from_min_max(97, 99).apply_mapping(&mapping),
    //     );
    //     assert_eq!(
    //         vec![SeedRange::new_from_min_max(50, 51)],
    //         SeedRange::new_from_min_max(98, 99).apply_mapping(&mapping),
    //     );
    //     assert_eq!(
    //         vec![
    //             SeedRange::new_from_min_max(51, 51),
    //             SeedRange::new_from_min_max(100, 101),
    //         ],
    //         SeedRange::new_from_min_max(99, 101).apply_mapping(&mapping),
    //     );
    //     assert_eq!(
    //         vec![SeedRange::new_from_min_max(101, 102),],
    //         SeedRange::new_from_min_max(101, 102).apply_mapping(&mapping),
    //     );
    // }

    // #[test]
    // fn test_random_ranges() {
    //     let mut rng = ChaCha12Rng::seed_from_u64(4711);
    //     for i in 0..100 {
    //         println!("\n<<< TEST CASE {i} >>>");
    //         let seed_range = SeedRange::new_random(&mut rng);
    //         seed_range.print('S');
    //         let mapping = RangeMapping::new_random(&mut rng);
    //         mapping.print();

    //         println!("Seed ranges mapped into:");
    //         let mapped_range = seed_range.apply_mapping(&mapping);
    //         for m in &mapped_range {
    //             m.print('S');
    //         }
    //         assert_eq!(
    //             seed_range.len,
    //             mapped_range.iter().map(|range| range.len).sum()
    //         );
    //     }
    // }

    #[test]
    fn test_ex1() {
        let ex1 = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(35, solve_p1(ex1));
        assert_eq!(46, solve_p2(ex1));
    }
}
