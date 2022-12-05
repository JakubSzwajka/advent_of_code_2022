pub mod common;

#[derive(Debug)]
struct Sections {
    pub from: u32,
    pub to: u32,
}

#[derive(Debug)]
struct ElvePair {
    elve_a_sections: Sections,
    elve_b_sections: Sections,
}

impl Sections {
    fn len(&self) -> u32 {
        return self.to - self.from;
    }
}

impl ElvePair {
    fn do_sections_fully_overlap(&self) -> bool {
        if self.elve_a_sections.len() > self.elve_b_sections.len() {
            return self.elve_a_sections.from <= self.elve_b_sections.from
                && self.elve_a_sections.to >= self.elve_b_sections.to;
        } else if self.elve_a_sections.len() < self.elve_b_sections.len() {
            return self.elve_b_sections.from <= self.elve_a_sections.from
                && self.elve_b_sections.to >= self.elve_a_sections.to;
        } else {
            return self.elve_a_sections.from == self.elve_b_sections.from
                && self.elve_a_sections.to == self.elve_b_sections.to;
        }
    }

    fn do_sections_overlap_at_all(&self) -> bool {
        return (self.elve_a_sections.to >= self.elve_b_sections.from
            && self.elve_a_sections.from <= self.elve_b_sections.to)
            || (self.elve_b_sections.to >= self.elve_a_sections.from
                && self.elve_b_sections.from <= self.elve_a_sections.to);
    }
}

fn main() {
    let args = common::read_args();
    let pairs = common::read_file(&args[1]);

    let mut overlapping_pair_count: u32 = 0;

    for pair in pairs {
        let pair_data: &str = pair.as_ref().expect("valid elve pair data").as_str();
        let pair_data: Vec<&str> = pair_data.split(",").collect();
        let elve_a_section_data: Vec<&str> = pair_data[0].split("-").collect();
        let elve_b_section_data: Vec<&str> = pair_data[1].split("-").collect();

        let elve_pair = ElvePair {
            elve_a_sections: Sections {
                from: elve_a_section_data[0].parse().unwrap(),
                to: elve_a_section_data[1].parse().unwrap(),
            },
            elve_b_sections: Sections {
                from: elve_b_section_data[0].parse().unwrap(),
                to: elve_b_section_data[1].parse().unwrap(),
            },
        };

        if elve_pair.do_sections_overlap_at_all() {
            overlapping_pair_count = overlapping_pair_count + 1;
            dbg!(&elve_pair);
        }
    }

    dbg!(overlapping_pair_count);
}
