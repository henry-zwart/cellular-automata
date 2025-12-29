//pub trait Neighbors {
//    type Output;
//
//    fn neighbors(&self, idx: usize) -> Self::Output;
//}
//
//pub struct Rule<const C: u8, const N: usize> {
//    val: u16,
//}
//
//impl<const C: u8, const N: usize> Rule<C, N> {
//    pub fn new(val: u16) -> Self {
//        Self { val }
//    }
//
//    pub fn map(&self, neighborhood: &[u8; N]) -> u8 {
//        let mut idx = 0;
//        for n in neighborhood {
//            idx = (idx | n) << log2(C)
//        }
//        ((self.val >> idx) & 0b1) as u8
//    }
//}
//
//const fn num_bits<T>() -> usize {
//    std::mem::size_of::<T>() * 8
//}
//
//const fn log2(x: u8) -> u8 {
//    (num_bits::<u8>() as u32 - x.leading_zeros() - 1) as u8
//}
//
//#[cfg(test)]
//mod test {
//    use super::*;
//
//    #[test]
//    fn nbits() {
//        assert_eq!(log2(2), 1);
//        assert_eq!(log2(3), 2);
//        assert_eq!(log2(4), 2);
//    }
//}
