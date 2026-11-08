pub mod bitpack;
pub use crate::bitpack::fitss;
pub use crate::bitpack::fitsu;
pub use crate::bitpack::gets;
pub use crate::bitpack::getu;
pub use crate::bitpack::news;
pub use crate::bitpack::newu;

#[cfg(test)]
mod tests {
    use crate::{bitpack};

    static SIGNED: [i64; 3] = [-11, 3, 2];
    static FITSS: [(u64, bool); 3] = [(5, true), (2, true), (1, false)];
    static GETS: [(u64, u64, u64, Option<i64>); 3] = [
        (
            0b0000000000000000000000000000000000000000000001101100000000000000,
            5,
            14,
            Some(-11)
        ),
        (
            0b0000000000000000000000000000000000000000000000000000000001101100,
            6,
            2,
            Some(27)
        ),
        (
            0b0000000000000000000000000000000000000000000000000000000000000000,
            32,
            33,
            None
        ),
    ];
    static UNSIGNED: [u64; 3] = [1, 8, 27];
    static FITSU: [(u64, bool); 3] = [(5, true), (3, false), (5, true)];
    static GETU: [(u64, u64, u64, Option<u64>); 3] = [
        (
            0b0000000000000000000000000000000000000000000001101100000000000000,
            5,
            14,
            Some(27)
        ),
        (
            0b0000000000000000000000000000000000000000000000000000000001101100,
            4,
            2,
            Some(11)
        ),
        (
            0b0000000000000000000000000000000000000000000000000000000000000000,
            32,
            33,
            None
        ),
    ];

    #[test]
    fn test_fitss() {
        for index in 0..SIGNED.len() {
            assert_eq!(bitpack::fitss(SIGNED[index], FITSS[index].0), FITSS[index].1);
        }
    }

    #[test]
    fn test_gets() {
        for index in 0..GETS.len() {
            assert_eq!(bitpack::gets(GETS[index].0, GETS[index].1, GETS[index].2), GETS[index].3);
        }
    }

    #[test]
    fn test_news() {
        assert_eq!(
            bitpack::news(
                0b0000000000000000000000000000000000000000000001101100000000000000,
                6,
                2,
                27
            ),
            Some(0b0000000000000000000000000000000000000000000001101100000001101100)
        );
        assert_eq!(
            bitpack::news(
                0b0000000000000000000000000000000000000000000001101100000000000000,
                4,
                2,
                27
            ),
            None
        );
    }

    #[test]
    fn test_fitsu() {
        for index in 0..UNSIGNED.len() {
            assert_eq!(bitpack::fitsu(UNSIGNED[index], FITSU[index].0), FITSU[index].1);
        }
    }

    #[test]
    fn test_getu() {
        for index in 0..GETU.len() {
            assert_eq!(bitpack::getu(GETU[index].0, GETU[index].1, GETU[index].2), GETU[index].3);
        }
    }

    #[test]
    fn test_newu() {
        assert_eq!(
            bitpack::newu(
                0b0000000000000000000000000000000000000000000001101100000000000000,
                6,
                2,
                27
            ),
            Some(0b0000000000000000000000000000000000000000000001101100000001101100)
        );
        assert_eq!(
            bitpack::newu(
                0b0000000000000000000000000000000000000000000001101100000000000000,
                4,
                2,
                27
            ),
            None
        );
    }
}
