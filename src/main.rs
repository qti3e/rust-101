fn main() {
    let mut list1 = vec![1, 2];
    let list2 = vec![3, 4, 5];

    let mut other: Vec<&i32> = vec![];

    append_by_ref(&mut other, &list1);
    append_by_ref(&mut other, &list2);
    append_by_ref(&mut other, &list1);

    dbg!(&other);
    dbg!(&list1);

    // list1.push(100);
    // ^-- this will not work!

    drop(other);

    list1.push(100);
}

fn append_by_ref<'a, 'b, T>(target: &'a mut Vec<&'b T>, source: &'b Vec<T>) {
    for x in source.iter() {
        target.push(x);
    }
}
