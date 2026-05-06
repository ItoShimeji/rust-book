enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    {
        let home = IpAddrKind::V4(127, 0, 0, 1);
        let loooback = IpAddrKind::V6(String::from("::1"));
    }

    {
        let some_number = Some(5);
        let some_cha = Some('e');

        // None の時は Some の型を提示する
        let absent_number: Option<i32> = None;
    }
}
