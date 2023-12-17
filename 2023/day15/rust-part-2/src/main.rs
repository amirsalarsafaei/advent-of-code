use atoi::atoi;

#[derive(Debug)]
struct Lens {
    label: String,
    focal_len: u32,
}

impl Lens {
    fn from(a: &str) -> Lens {
        let (label, focal_len_str) = a.split_once("=").unwrap();
        let focal_len = atoi(focal_len_str.as_bytes()).unwrap();
        return Lens {
            label: String::from(label),
            focal_len,
        };
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let mut boxes: Vec<Vec<Lens>> = (0..256)
        .map(|_a| Vec::new())
        .collect::<Vec<_>>();

    input.split(",").for_each(|label| {
        if label.chars().last().unwrap() == '-' {
            let box_number = hash_algorithm(&label[0..label.len() - 1]) as usize;

            if let Some(idx) = boxes[box_number]
                .iter()
                .position(|a| a.label == label[0..label.len() - 1]) {
                boxes[box_number].remove(idx);
            }
        } else {
            let lens = Lens::from(label);
            let mut found = false;
            let box_number = hash_algorithm(lens.label.as_str()) as usize;

            for i in 0..boxes[box_number].len() {
                if boxes[box_number][i].label == lens.label {
                    boxes[box_number][i].focal_len = lens.focal_len;
                    found = true;
                    break;
                }
            }

            if !found {
                boxes[box_number].push(lens)
            }
        }
    });

    println!("{}",
             boxes.iter().enumerate().map(
                 |(box_num, lenses)| {
                     return lenses.iter()
                         .enumerate()
                         .fold(0i32, |a, (slot_num, lens)| {
                             a + (
                                 (box_num + 1) as i32
                                     * (slot_num + 1) as i32
                                     * lens.focal_len as i32
                             )
                         });
                 }
             ).fold(0i32, |a, b| a + b)
    )
}

fn hash_algorithm(s: &str) -> i32 {
    s.chars().fold(0i32, |a, b| { ((a + b as i32) * 17) % 256 })
}
