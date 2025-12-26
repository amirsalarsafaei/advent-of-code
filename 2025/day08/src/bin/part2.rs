use std::mem;

use day08::{get_problem, InputType,  Circuit,  get_connection_pairs};


fn main() {
    let mut problem = get_problem(InputType::Test).expect("Failed to get problem");

    let mut circuits = (0..problem.junction_boxes.len()).map(|jb| {
        Circuit {
            junction_boxes: vec![jb]
        }
    }).collect::<Vec<Circuit>>();

    for (_, i, j) in get_connection_pairs(&problem) {
        let dbi = problem.junction_boxes[i];
        let dbj = problem.junction_boxes[j];

        if dbi.circuit_id == dbj.circuit_id {
            continue;
        }

        let cid_i = dbi.circuit_id;
        let cid_j = dbj.circuit_id;

        let ci_len = circuits[cid_i].junction_boxes.len();
        let cj_len = circuits[cid_j].junction_boxes.len();

        let (src_id, dst_id) = if ci_len < cj_len {
            (cid_i, cid_j) 
        } else { 
            (cid_j, cid_i) 
        }; 
        
        let mut boxes_to_move = mem::take(&mut circuits[src_id].junction_boxes);
        circuits[dst_id].junction_boxes.append(&mut boxes_to_move.clone());
        for &jb_index in &boxes_to_move {
            problem.junction_boxes[jb_index].circuit_id = dst_id;
        }
        boxes_to_move.clear();

        if circuits[dst_id].junction_boxes.len() == problem.junction_boxes.len() {
            println!("i: {}, j: {}, multipication: {}", dbi, dbj, dbi.x * dbj.x);
            return;
        }
    }
}
