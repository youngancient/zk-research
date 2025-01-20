use ark_bn254::Fq;
use shamir::shamir;

fn main() {
    let secret: Fq = Fq::from(1000);
    let threshold = 4;
    let shares_no = 10;
    let shares = shamir::share_secret(secret, threshold, shares_no);

    let first_4_shares: Vec<(Fq, Fq)> = shares.iter().take(3).cloned().collect();
    let recovered_secret = shamir::recover_secret(first_4_shares);

    println!("Recovered secret: {}", recovered_secret);
}
