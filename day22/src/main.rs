#[derive(Debug, Copy, Clone)]
struct Step {
    action: bool,
    cuboid: Cuboid,
}

#[derive(Debug, Copy, Clone)]
struct Cuboid {
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
}

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let steps = parse_input(input);

    let mut cuboids: Vec<Cuboid> = Vec::new();
    for step in steps {
        let mut new_cuboids: Vec<Cuboid> = Vec::new();
        for other in cuboids {
            new_cuboids.append(&mut other.intersect(&step.cuboid));
        }
        if step.action {
            new_cuboids.push(step.cuboid);
        }
        cuboids = new_cuboids;
    }

    (
        cuboids.iter().map(|c| c.volume_part1()).sum(),
        cuboids.iter().map(|c| c.volume_part2()).sum(),
    )
}

fn parse_input(input: &'static str) -> Vec<Step> {
    let mut ret = Vec::new();
    for line in input.trim_end().split('\n') {
        ret.push(parse_line(line));
    }
    ret
}

fn parse_line(line: &'static str) -> Step {
    let mut s = line.split(' ');
    let action = match s.next().unwrap() {
        "on" => true,
        "off" => false,
        _ => unreachable!(),
    };

    let mut s = s.next().unwrap().split(',');
    let mut x = s.next().unwrap().strip_prefix("x=").unwrap().split("..");
    let mut y = s.next().unwrap().strip_prefix("y=").unwrap().split("..");
    let mut z = s.next().unwrap().strip_prefix("z=").unwrap().split("..");
    Step {
        action,
        cuboid: Cuboid {
            x: (
                x.next().unwrap().parse().unwrap(),
                x.next().unwrap().parse().unwrap(),
            ),
            y: (
                y.next().unwrap().parse().unwrap(),
                y.next().unwrap().parse().unwrap(),
            ),
            z: (
                z.next().unwrap().parse().unwrap(),
                z.next().unwrap().parse().unwrap(),
            ),
        },
    }
}

impl Cuboid {
    fn intersect(mut self, other: &Cuboid) -> Vec<Cuboid> {
        let mut ret = Vec::new();
        if (self.x.0 <= other.x.1 && self.x.1 >= other.x.0)
            && (self.y.0 <= other.y.1 && self.y.1 >= other.y.0)
            && (self.z.0 <= other.z.1 && self.z.1 >= other.z.0)
        {
            if self.x.0 < other.x.0 {
                ret.push(Cuboid {
                    x: (self.x.0, other.x.0 - 1),
                    y: self.y,
                    z: self.z,
                });
                self.x.0 = other.x.0;
            }
            if self.x.1 > other.x.1 {
                ret.push(Cuboid {
                    x: (other.x.1 + 1, self.x.1),
                    y: self.y,
                    z: self.z,
                });
                self.x.1 = other.x.1;
            }
            if self.y.0 < other.y.0 {
                ret.push(Cuboid {
                    x: self.x,
                    y: (self.y.0, other.y.0 - 1),
                    z: self.z,
                });
                self.y.0 = other.y.0;
            }
            if self.y.1 > other.y.1 {
                ret.push(Cuboid {
                    x: self.x,
                    y: (other.y.1 + 1, self.y.1),
                    z: self.z,
                });
                self.y.1 = other.y.1;
            }
            if self.z.0 < other.z.0 {
                ret.push(Cuboid {
                    x: self.x,
                    y: self.y,
                    z: (self.z.0, other.z.0 - 1),
                });
                self.z.0 = other.z.0;
            }
            if self.z.1 > other.z.1 {
                ret.push(Cuboid {
                    x: self.x,
                    y: self.y,
                    z: (other.z.1 + 1, self.z.1),
                });
                self.z.1 = other.z.1;
            }
        } else {
            ret.push(self);
        }
        ret
    }

    fn volume_part1(&self) -> usize {
        diff_part1(self.x) * diff_part1(self.y) * diff_part1(self.z)
    }

    fn volume_part2(&self) -> usize {
        (self.x.1 - self.x.0 + 1) as usize
            * (self.y.1 - self.y.0 + 1) as usize
            * (self.z.1 - self.z.0 + 1) as usize
    }
}

fn diff_part1(s: (isize, isize)) -> usize {
    let (mut a, mut b) = s;
    if (a < -50 && b < -50) || (a > 50 && b > 50) {
        return 0;
    }
    if a < -50 {
        a = -50;
    } else if a > 50 {
        a = 50;
    }
    if b < -50 {
        b = -50;
    } else if b > 50 {
        b = 50;
    }
    (b - a + 1) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_example() {
        let steps = parse_input(include_str!("../input-example1"));
        assert_eq!(steps.len(), 4);
    }

    #[test]
    fn test_input_example1() {
        let (part1_answer, _) = run(include_str!("../input-example1"));
        assert_eq!(part1_answer, 39);
    }

    #[test]
    fn test_input_example2() {
        let (part1_answer, _) = run(include_str!("../input-example2"));
        assert_eq!(part1_answer, 590784);
    }

    #[test]
    fn test_input_example3() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example3"));
        assert_eq!(part1_answer, 474140);
        assert_eq!(part2_answer, 2758514936282235);
    }

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 607657);
        assert_eq!(part2_answer, 1187742789778677);
    }
}
