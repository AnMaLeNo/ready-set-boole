fn adder(a: u32, b: u32) -> u32 {
    let mut next_retenu: u32;
    let mut retenu: u32 = 0;
    let mut resultat: u32 = 0;
    for i in 0..32 {
        next_retenu = ((a & (1 << i)) & (b & (1 << i))) << 1;
        if next_retenu == 0 {
            next_retenu = (((a & (1 << i)) | (b & (1 << i))) & retenu) << 1;
            resultat = resultat | (((a & (1 << i)) | (b & (1 << i))) ^ retenu);
        } else {
            resultat = resultat | retenu;
        }
        retenu = next_retenu;
    }
    resultat
}

fn main() {
    for a in 0..1000u32 {
        for b in u32::MAX-1000..u32::MAX {
            if a.wrapping_add(b) != adder(a, b) {
                println!("{} + {} != {}", a, b, adder(a, b));
            }
        }
    }
}
