use shamir::shamir;

fn main() {
    let secret = 340282366920938463463374607431768211455;
    let threshold = 4;
    let shares_no = 10;
    let shares = shamir::share_secret(secret, threshold, shares_no);
    println!("{:?}", shares);
}
