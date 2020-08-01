struct Solution();
impl Solution {
    /*
     * Replace good_luck
     */
    pub fn good_luck() -> Option<()> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(None, Solution::good_luck());
    }
}
