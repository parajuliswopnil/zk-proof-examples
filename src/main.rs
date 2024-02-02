use rand::prelude::*;

/// To prove that square root of y exists in the group 1-N
static Y: u64= 5;
static ROOT_Y: u64 = 4;
static N: u64 = 11;


/// This is a prover that proves a certain square root of `y` belongs to the group 1 < y < N such that y = x ^ 2 mod N which is to say x belongs to ZN
fn interactive_zk_proofs(one_or_zero: u8) -> (u64, u64){
    // let us chose a random number r so that to prove 
    match one_or_zero {
          1u8 =>  {
            let mut random = rand::thread_rng();

            let r:u64 = random.gen_range(1..11);
            let s = r.pow(2) % N;
            return ((r*ROOT_Y) % N, s);
        },
          0u8 => {
            let mut random = rand::thread_rng();

            let rng:u64 = random.gen_range(1..11);

            let r = rng % N;

            let s = r.pow(2) % N;

            return (r % N, s);
        },
          _ => return (0u64, 0u64)
    };
    
}

fn verify_zkp(one_or_zero: u8, z: u64, s: u64) {
    match one_or_zero {
        1u8 => {
            println!("heads or tails {}", one_or_zero);
            let zsquare = z.pow(2) % N;
            let sypow1 = (s * Y) % N;
            println!("{} {}", zsquare, sypow1);
            assert_eq!(zsquare, sypow1);
        }
        0u8 => {
            println!("heads or tails {}", one_or_zero);
            let zsquare = z.pow(2) % N;
            let sypow0 = s % N;
            println!("{} {}", zsquare, sypow0);
            assert_eq!(zsquare, sypow0);
        }
        _ => println!("error")
    }
}



fn main() {
    let mut random = rand::thread_rng();
    let rng: u8 = random.gen();
    let one_or_zero = rng % 2u8;


    let (z, s) = interactive_zk_proofs(one_or_zero);
    verify_zkp(one_or_zero, z, s)
}

#[test]
fn simulate() {
    let mut i:u8 = 0;
    loop {
        if i == 100 {
            break;
        }
        let one_or_zero = i % 2;
        let (z, s) = interactive_zk_proofs(one_or_zero);
        verify_zkp(one_or_zero, z, s);
        i += 1; 
    }
}
