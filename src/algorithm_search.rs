fn run() {
    // 線形探索
    let mut v = vec![1, 2, 3, 4, 5];
    let mut found = false;
    let mut i = 0;
    let mut index = 0;
    let target = 3;
    while i < v.len() {
        if v[i] == target {
            found = true;
            index = i;
            break;
        }
        i += 1;
    }
}
