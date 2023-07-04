struct SectionBoundary {
    start: u32,
    end: u32,
}

pub fn solve(str: &String) {
    let lines = str.lines();
    let mut sum_of_overlaps = 0;
    let mut index = 0;

    for line in lines {
        let boundaries = get_section_boundaries(line);
        let mut overlap = false;
        for boundary in &boundaries {
            if overlap {
                continue;
            };

            println!("Checking boundary: {}-{}", boundary.start, boundary.end);
            for to_check in &boundaries {
                if std::ptr::eq(boundary, to_check) || overlap {
                    continue;
                }

                println!("Checking against: {}-{}", to_check.start, to_check.end);
                if boundary.start >= to_check.start && boundary.end <= to_check.end {
                    sum_of_overlaps += 1;
                    overlap = true;
                }
            }
            index += 1;
        }
    }
    println!("Lines read: {}", index);

    println!("Sum of overlaps: {}", sum_of_overlaps);
}

pub fn solve_second(str: &String) {
    let lines = str.lines();
    let mut sum_of_overlaps = 0;
    let mut index = 0;

    for line in lines {
        let boundaries = get_section_boundaries(line);
        let mut overlap = false;
        for boundary in &boundaries {
            if overlap {
                continue;
            };

            println!("Checking boundary: {}-{}", boundary.start, boundary.end);
            for to_check in &boundaries {
                if std::ptr::eq(boundary, to_check) || overlap {
                    continue;
                }

                println!("Checking against: {}-{}", to_check.start, to_check.end);
                if (boundary.start >= to_check.start && boundary.end <= to_check.end) ||
                   (boundary.start <= to_check.end && boundary.end >= to_check.start)
                {
                    sum_of_overlaps += 1;
                    overlap = true;
                }
            }

            index += 1;
        }
    }

    println!("Lines read: {}", index);
    println!("Sum of overlaps: {}", sum_of_overlaps);
}

fn get_section_boundaries(line: &str) -> Vec<SectionBoundary> {
    let elves: Vec<&str> = line.split(",").collect();
    let mut boundaries = Vec::new();

    for elf in elves {
        let elf_boundaries: Vec<&str> = elf.split("-").collect();
        let lower: u32 = elf_boundaries[0].parse().unwrap();
        let upper: u32 = elf_boundaries[1].parse().unwrap();

        let boundary = SectionBoundary {
            start: lower,
            end: upper,
        };

        boundaries.push(boundary);
    }

    return boundaries;
}
