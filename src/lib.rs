#[allow(dead_code)]
struct Solution();

impl Solution {
    /*
     * Replace good_luck
     */
    #[allow(dead_code)]
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
