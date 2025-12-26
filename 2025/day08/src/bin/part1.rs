use std::mem;

use day08::{Circuit, InputType, get_connection_pairs, get_problem};


fn main() {
    let mut problem = get_problem(InputType::Sample).expect("Failed to get problem");

    let mut circuits = (0..problem.junction_boxes.len()).map(|jb| {
        Circuit {
            junction_boxes: vec![jb]
        }
    }).collect::<Vec<Circuit>>();
    
    let mut connections = 0;


    for (_, i, j) in get_connection_pairs(&problem) {
        let dbi = problem.junction_boxes[i];
        let dbj = problem.junction_boxes[j];

        connections += 1;
        if connections == problem.connections {
            break;
        }

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


    }

    circuits.retain(|c| !c.junction_boxes.is_empty());
    circuits.sort_by(|a, b| a.junction_boxes.len().cmp(&b.junction_boxes.len()).reverse());
    let res = circuits[0].junction_boxes.len() * circuits[1].junction_boxes.len() *
        circuits[2].junction_boxes.len();

    println!("circuits({}): {:?}", circuits.len(), circuits);

    println!("Result: {}", res);
}
