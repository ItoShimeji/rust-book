use adder::add;

#[test]
fn it_adds() {
    let result = add(2, 4);
    assert_eq!(result, 6);
}
