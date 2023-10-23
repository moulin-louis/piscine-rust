fn swap_reverse_arr(boxes: &mut [[u32; 2]]) {
    let mut swapped: bool;
    for i in 0..boxes.len() - 2 {
        swapped = false;
        for j in 0..(boxes.len() - i - 1) {
            if boxes[j] > boxes[j + 1] {
                boxes.swap(j, j + 1);
                swapped = true;
            }
        }
        if swapped == false{
            break;
        }
    }
    for i in 0..(boxes.len() / 2) {
        boxes.swap(i, boxes.len() - i -1);
    }
}

pub fn sort_boxes(boxes: &mut [[u32; 2]]) {
    if boxes.is_empty() {
        panic!("Boxes is empty");
    }
    // println!("Boxes before swap {boxes:?}");
    swap_reverse_arr(boxes);
    // println!("Boxes after swap {boxes:?}");
    let mut prev_w = boxes[0][0];
    let mut prev_h = boxes[0][1];
    for boxe in &boxes[1..boxes.len()] {
        if boxe[0] > prev_w {
            panic!("Cant fit every box!");
        }
        if boxe[1] > prev_h {
            panic!("Cant fit every box!");
        }
        prev_w = boxe[0];
        prev_h = boxe[1];
    }
}

#[cfg(test)]
mod tests {
    use crate::sort_boxes;

    #[test]
    fn mandatory_test() {
        let mut boxes: [[u32; 2]; 5] = [[3, 3], [4, 3], [1, 0], [5, 7], [3, 3]];
        sort_boxes(&mut boxes);
        assert_eq!(boxes, [[5, 7], [4, 3], [3, 3], [3, 3], [1, 0]]);
    }
}
