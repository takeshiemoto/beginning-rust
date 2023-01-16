#[allow(dead_code)]
enum E {
    Case1(u32),
    Case2(char),
    Case3(i64, bool),
}

pub fn run() {
    let v = E::Case3(1234, true);
    // match v {
    //     E::Case3(n, b) => {
    //         if b {
    //             println!("{}", n)
    //         }
    //     }
    //     _ => {}
    // }

    if let E::Case3(n, b) = v {
        if b {
            println!("{}", n)
        }
    }
}
