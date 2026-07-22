use std::{
    future::Future,
    pin::{Pin, pin},
    time::Duration,
};

fn main() {
    trpl::block_on(async {
        // future は後で実行する状態機械
        // そのため、以下のように async block で future を作ったあとに
        // これからは heap 上で移動しないことを pin で明示する
        let first = pin!(async {
            trpl::sleep(Duration::from_millis(300)).await;
            println!("first completed");
        });

        let second = pin!(async {
            trpl::sleep(Duration::from_millis(100)).await;
            println!("second completed");
        });

        let third = pin!(async {
            trpl::sleep(Duration::from_millis(200)).await;
            println!("third completed");
        });

        // 各 async ブロックは、Output が同じ () でも別々の匿名型になる。
        // dyn Future に型消去することで、同じ Vec に格納できる。
        //
        // Pin<P> が固定するのは P 自体ではなく、P の Deref 先。
        // ここでは &mut ポインタではなく、その参照先の Future が固定される。
        // Pin<&mut ...> 自体は Vec の中へ move してもよい。
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![first, second, third];

        // ここで直接 await しているのは個々の Future ではなく JoinAll
        // future は内部参照が含まれるため、heap 上で移動してはいけない
        // そのため、Pin で固定する必要がある
        // 要素の Future は join_all に渡す前に pin 済みである必要がある
        // 普段使用する大体の型は move が安全であるため、Uppin という marker trait が実装されている
        trpl::join_all(futures).await;

        println!("all completed");
    });
}
