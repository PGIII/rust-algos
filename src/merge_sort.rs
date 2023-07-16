pub fn merge_sort<T>(list: &Vec<T>) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    //if vec is less than 2 in len we cant sort
    if list.len() < 2 {
        list.clone()
    } else {
        //split and mergesort each half
        let middle = list.len() / 2;
        let m1 = merge_sort(&list[..middle].to_vec());
        let m2 = merge_sort(&list[middle..].to_vec());
        merge(&m1, &m2)
    }
}

fn merge<T>(list1: &Vec<T>, list2: &Vec<T>) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    let mut out = vec![];
    let mut i1 = 0;
    let mut i2 = 0;
    while i1 < list1.len() && i2 < list2.len() {
        if list1[i1] < list2[i2] {
            out.push(list1[i1]);
            i1 += 1;
        } else {
            out.push(list2[i2]);
            i2 += 1;
        }
    }
    while i1 < list1.len() {
        out.push(list1[i1]);
        i1 += 1;
    }
    while i2 < list2.len() {
        out.push(list2[i2]);
        i2 += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i32() {
        let unsorted = vec![3, 0, 2, 1];
        let sorted = vec![0, 1, 2, 3];
        let result = merge_sort(&unsorted);
        assert_eq!(sorted, result);
    }
    #[test]
    fn char() {
        let unsorted = vec!['d', 'a', 'c', 'b'];
        let sorted = vec!['a', 'b', 'c', 'd'];
        let result = merge_sort(&unsorted);
        assert_eq!(sorted, result);
    }
}
