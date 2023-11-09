pub fn bubble_sort<T>(array: &mut [T])
where
    T: PartialOrd,
{
    for i in 0..array.len() - 1 {
        for j in (i + 1)..array.len() {
            if array[j] < array[i] {
                array.swap(i, j);
            }
        }
    }
}

pub trait BubbleSort {
    fn bubble_sort(&mut self);
}

impl<T: PartialOrd> BubbleSort for [T] {
    fn bubble_sort(&mut self) {
        bubble_sort(self)
    }
}

#[cfg(test)]
mod tests {
    use super::BubbleSort as _;

    #[test]
    fn it_works_i8() {
        let array: &mut [i8] = &mut [3, 1, 2];
        array.bubble_sort();
        assert_eq!(array, [1, 2, 3]);
    }

    #[test]
    fn it_works_u8() {
        let array: &mut [u8] = &mut [3, 1, 2];
        array.bubble_sort();
        assert_eq!(array, [1, 2, 3]);
    }

    #[test]
    fn it_works_u8_vec() {
        let mut array: Vec<u8> = vec![3, 1, 2];
        array.bubble_sort();
        assert_eq!(array, vec![1, 2, 3]);
    }

    #[test]
    fn it_works_char_vec() {
        let mut array: Vec<char> = vec!['🥚', '🐣', '🐔'];
        array.bubble_sort();
        assert_eq!(array, vec!['🐔', '🐣', '🥚']);
    }

    #[test]
    fn it_works_str_vec() {
        let mut array: Vec<&str> = vec!["DOT", "ETH", "BTC"];
        array.bubble_sort();
        assert_eq!(array, vec!["BTC", "DOT", "ETH"]);
    }

    #[test]
    fn it_works_bool_vec() {
        let mut array: Vec<bool> = vec![true, false];
        array.bubble_sort();
        assert_eq!(array, vec![false, true]);
    }

    #[test]
    fn it_works_option_vec() {
        let mut array: Vec<Option<()>> = vec![Some(()), None];
        array.bubble_sort();
        assert_eq!(array, vec![None, Some(())]);
    }
}
