
fn negyedikFeladat() -> bool{
    let tomb:[i32;20] = [3, 1, -6, -4, 3, 0, 2, 3, -1, -1, -3, -2, 6, 1, -2, -1, 2, -3, 3, 4];
    let mut i = 0;
    while i<tomb.len() && tomb[i] != 0 {
        i+= 1;
        
    }
    if i<tomb.len() {
        true
    }
    else {
        false
    }
}

fn harmadikFeladat() -> i32{
    let tomb: [i32;20] = [3, 1, -6, -4, 3, 0, 2, 3, -1, -1, -3, -2, 6, 1, -2, -1, 2, -3, 3, 4];
    let mut maxi = 0;
    let mut counter = 0;
    for i in 1..tomb.len()  {
        
        if tomb[i] > tomb[maxi] {
            maxi = i;
            counter = 1;   
        }
        if tomb[i] == tomb[maxi].abs() {
            counter += 1;
        }
        
    }
    counter
}

fn masodikFeladat() -> i32{
    let tomb: [i32; 20] = [3, 1, -4, -4, 3, 0, 2, 3, -1, -1, -3, -2, 4, 1, -2, -1, 2, -3, 3, 4];
    let mut elore: i32 = 0;
    let mut maxElore : i32 = 0;
    for i in 0..tomb.len()  {
        if tomb[i] >= 0 {
            elore += 1;
        }else {
            elore = 0;
        } if elore > maxElore {
            maxElore = elore;
        }
    }
    maxElore
}


fn elsoFeladat() -> f64{
    let tomb: [i32; 20] = [3, 1, -4, -4, 3, 0, 2, 3, -1, -1, -3, -2, 4, 1, -2, -1, 2, -3, 3, 4]; 
    let mut teljes: i32 = 0;
    let mut mozdulas: i32 = 0;  
    for i in 0..tomb.len()  {
        teljes += tomb[i].abs();
        mozdulas += tomb[i];
        
    }
    let elmozdulas = mozdulas.abs();

    let szazelek =((teljes as f64 - elmozdulas as f64)/teljes as f64)*100.0;
    szazelek
}



fn main() {
    // print!("{}",elsoFeladat());
    println!("{}",masodikFeladat());
    println!("{}",harmadikFeladat());
    println!("{}",negyedikFeladat());
}
