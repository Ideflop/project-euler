/* 
Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.
How many such routes are there through a 20×20 grid?
*/

fn main() {
    let grid = vec![20]; 

    let row = 2 * grid[0];
    let col = row / 2;
    
    pascal_triangle(row, col);

    /* this doesn't work because value is too large
    let value: usize = (1..=row).product();
    let a: usize = (1..=col).product();
    let b: usize = (1..=row - col).product();
    println!("{}", value / (a * b));
    */
}

fn pascal_triangle(row: usize, col: usize) {
    for i in 0..row + 1{
        let mut c = 1;
        for k in 0..i+1 {
            c = c * (i-k)/(k+1);
            if k == col - 1 && i == row {
                println!("{}", c);
            }
        }
    }
}
