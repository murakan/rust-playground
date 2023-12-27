// snippet of code @ 2023-09-12 20:43:11

// === Rust Playground ===
// This snippet is in: ~/.emacs.d/rust-playground/at-2023-09-12-204308/

// Execute the snippet: C-c C-c
// Delete the snippet completely: C-c k
// Toggle between main.rs and Cargo.toml: C-c b

const VALUE_1D: [i32; 4] = [1, 2, 3, 4];
const VALUE_2D: [[i32; 3]; 4] = [[1, 2, 3], [4, 5, 6], [7, 8, 9], [10, 11, 12]];
const VALUE_3D: [[[i32; 2]; 3]; 4] = [
    [[1, 2], [3, 4], [5, 6]],
    [[7, 8], [9, 10], [11, 12]],
    [[13, 14], [15, 16], [17, 18]],
    [[19, 20], [21, 22], [23, 24]],
];

fn main() {
    println!("Results:");
    println!("1D: {:?}", VALUE_1D);
    println!("2D: {:?}", VALUE_2D);
    println!("3D: {:?}", VALUE_3D);
}
