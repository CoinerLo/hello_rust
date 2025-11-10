fn find_intersection<'a>(a: &'a [i32], b: &'a [i32]) -> &'a [i32] {
    let mut i = 0;
    let mut z = 0;
    let mut start = 0;

    while i < a.len() && z < b.len() {
        if a[i] == b[z] {
            if start == 0 {
                start = i + 1;
            }
            i += 1;
            z += 1;
        } else if a[i] < b[z] {
            if start > 0 {
                return &a[start - 1..i];
            }
            i += 1;
        } else {
            z += 1;
        }
    }

    if start > 0 {
        &a[start - 1..i]
    } else {
        &[]
    }
}

fn main() {
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [3, 4, 5, 6, 7];
    assert_eq!(find_intersection(&arr1, &arr2), &[3, 4, 5]);

    let arr3 = [10, 20, 30];
    let arr4 = [25, 30, 35];
    assert_eq!(find_intersection(&arr3, &arr4), &[30]);

    let empty: &[i32] = &[];
    assert_eq!(find_intersection(empty, &arr1), &[]);

    {
        let v1 = vec![5, 10, 15, 20];
        let v2 = vec![15, 20, 25];
        assert_eq!(find_intersection(&v1, &v2), &[15, 20]);
    }

    println!("Все тесты прошли!");
}
