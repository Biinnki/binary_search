fn main() {
    let list: [u8; 5] = [0, 2, 5, 8, 14];
    let mut left: u8 = 0;
    let mut right: u8 = list.len() as u8;
    let mut middle: u8;
    
    let target: u8 = 13;

    let mut iter_counter: u8 = list.len() as u8;

    while true {
        middle = (left+right) / 2;        
        
        println!("left: {left} | middle: {middle} | right: {right}");

        if list[middle as usize] == target {
            print!("{middle} is the index of the target");
            break;
        }
        else if middle > target {
            right = middle;
        }
        else if middle < target {
            left = middle;
        }
        else if right < left {
            print!("Not found!");
            break;
        }

        iter_counter -= 1;
        if iter_counter <= 0 {
            println!("Something went wrong!");
            break;
        }
    }
}