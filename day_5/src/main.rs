fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input-1.txt")?;

    let almanac: Almanac = {
        let mut seeds: Vec<_> = vec![];
        let mut range_maps: Vec<_> = vec![];
        input.split("\n\n").enumerate().for_each(|(i, l)| {
            if i == 0 {
                // it's seeds
                seeds = l
                    .split(':')
                    .skip(1)
                    .next()
                    .expect("seeds always there")
                    .trim()
                    .split(' ')
                    .map(|n| n.parse::<u64>().expect("seed is num"))
                    .collect();
                eprintln!("parsed seeds: {:?}", seeds);
            } else {
                let mut range_map = RangeMap::new();
                l.split('\n').skip(1).for_each(|l| {
                    let nums: Vec<_> = l
                        .split(' ')
                        .map(|n| n.parse::<u64>().expect("range is num"))
                        .collect();
                    range_map.add_range(Range {
                        dest: *nums.get(0).expect("dest missing"),
                        src: *nums.get(1).expect("src missing"),
                        len: *nums.get(2).expect("len missing"),
                    });
                });
                range_maps.push(range_map);
            }

            println!("--{}--", l);
        });
        Almanac { seeds, range_maps }
    };
    eprintln!("almanac {:?}", almanac);
    eprintln!(
        "min location: {}",
        almanac
            .get_locations_for_seeds()
            .iter()
            .min()
            .expect("at least one should have")
    );

    Ok(())
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    range_maps: Vec<RangeMap>,
}

impl Almanac {
    fn get_locations_for_seeds(&self) -> Vec<u64> {
        eprintln!("range maps len {:?}", self.range_maps.len());

        self.seeds
            .iter()
            .map(|s| {
                let mut source = *s;

                self.range_maps.iter().for_each(|r| {
                    source = r.get_dest(source);
                });

                source
            })
            .collect()
    }
}

#[derive(Debug)]
struct RangeMap {
    ranges: Vec<Range>,
}

impl RangeMap {
    pub fn new() -> Self {
        Self { ranges: vec![] }
    }

    pub fn add_range(&mut self, range: Range) {
        self.ranges.push(range);
    }

    pub fn get_dest(&self, id: u64) -> u64 {
        self.ranges
            .iter()
            .find(|r| r.get_dest(id).is_some())
            .map(|r| r.get_dest(id).expect("find above"))
            .unwrap_or(id)
    }
}

#[derive(Debug)]
struct Range {
    dest: u64,
    src: u64,
    len: u64,
}

impl Range {
    pub fn new(dest: u64, src: u64, len: u64) -> Self {
        Range { dest, src, len }
    }

    pub fn get_dest(&self, src_id: u64) -> Option<u64> {
        if src_id >= self.src && src_id < self.src + self.len {
            Some(self.dest + (src_id - self.src))
        } else {
            None
        }
    }
}
