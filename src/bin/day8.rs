#[derive(Debug)]
struct Tree {
    height: i8,
    vis: bool,
    score: usize,
}

fn main() {
    let input = include_str!("../input/day8.txt");

    let stride = input.lines().next().unwrap().len();
    let mut data: Vec<Tree> = input
        .as_bytes()
        .iter()
        .flat_map(|&b| {
            (b != b'\n').then(|| Tree {
                height: b as i8 - 48,
                vis: false,
                score: 1,
            })
        })
        .collect();
    let num_strides = data.len() / stride;

    for i in 0..num_strides {
        mark_vis(data[i * stride..][..stride].iter_mut());
        mark_vis(data[i * stride..][..stride].iter_mut().rev());
        for j in 1..stride {
            mark_score(data[i * stride..][..stride].iter_mut(), j);
            mark_score(data[i * stride..][..stride].iter_mut().rev(), j);
        }
    }

    for i in 0..stride {
        mark_vis(data.iter_mut().skip(i).step_by(stride));
        mark_vis(data.iter_mut().rev().skip(i).step_by(stride));
        for j in 1..stride {
            mark_score(data.iter_mut().skip(i).step_by(stride), j);
            mark_score(data.iter_mut().rev().skip(i).step_by(stride), j);
        }
    }

    // part 1
    let part1 = data.iter().filter(|tree| tree.vis).count();
    assert_eq!(part1, 1789);

    // part 2
    let part2 = data.iter().map(|tree| tree.score).max().unwrap();
    assert_eq!(part2, 314820);
}

fn mark_vis<'a, I>(iter: I)
where
    I: Iterator<Item = &'a mut Tree>,
{
    let mut max = -1;
    for tree in iter {
        if tree.height > max {
            max = tree.height;
            tree.vis = true;
        }
    }
}

fn mark_score<'a, I>(mut iter: I, i: usize)
where
    I: Iterator<Item = &'a mut Tree>,
{
    let mut score = 0;
    let tree = iter.by_ref().nth(i).unwrap();
    for t in iter {
        score += 1;
        if t.height >= tree.height {
            break;
        }
    }
    tree.score *= score;
}
