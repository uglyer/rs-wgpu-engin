macro_rules! calculate {
    // 单个 `eval` 的模式
    (eval $e:expr) => {{
        {
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    }};

    // 递归地拆解多重的 `eval`
    (eval $e:expr, $(eval $es:expr),+) => {{
        fn run(){
            println!("run");
        }
        run();
        calculate! { eval $e }
        calculate! { $(eval $es),+ }
    }};
}

pub(crate) use calculate;
