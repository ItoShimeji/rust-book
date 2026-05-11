use std::sync::Arc;

fn main() {
    {
        // generics を用いて、要素の型を明示する
        let v: Vec<i32> = Vec::new();
    }

    {
        // vec! マクロを用いて、初期値で作成できる
        // この場合は型は推論される
        let v = vec![1, 2, 3];
    }

    {
        let mut v = Vec::new();

        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
    }

    {
        let v = vec![1, 2, 3, 4, 4];

        // この方法で範囲外アクセスをすると、panic になる
        // panic になって欲しいなら有用
        let third: &i32 = &v[2];
        println!("The third element is {third}");

        let third: Option<&i32> = v.get(2);
        match third {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element."),
        }

        // if let Some(third) = third {
        //     println!("The third element is {third}")
        // } else {
        //     println!("There is no third element.")
        // }
    }

    {
        let mut v = vec![1, 2, 3, 4, 5];

        // heap 上の i32 への参照が手に入る
        // i32 は Copy できるため、コンパイルエラーは起きない
        // &v[0] とすると、要素への imutable borrow が生じ、pushでエラーとなる
        let first = v[0];

        //　&v[0] だと、immutable borrow 中に mutation は禁止のため、エラー
        v.push(6);

        println!("The first element is: {first}")
    }

    {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            // dereference
            *i += 50;
        }
    }
}
