pub fn summarize(vec: Vec<f64>, p: usize) {
    let i = p * (vec.len() - 1);
    vec[i + 1];
}

pub fn run() {
    let vec = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
    let result = summarize(vec, 2);
}
