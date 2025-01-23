

#[cfg(test)]
pub mod tests{
    use super::*;
    use ark_bn254::Fq;

    #[test]
    pub fn test_test(){
        assert_eq!(Fq::from(2),Fq::from(2));
        assert_ne!(Fq::from(3),Fq::from(5));
    }
}