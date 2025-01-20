use shamir::shamir;
fn main(){
    
    let secret = 1000.0;
    let threshold = 4;
    let shares_no = 10;
    let shares = shamir::share_secret(secret, threshold, shares_no);

    let first_4_shares: Vec<(f64, f64)> = shares.iter().take(4).cloned().collect();
    let recovered_secret = shamir::recover_secret(first_4_shares);
    println!("Recovered secret: {}", recovered_secret);
}
