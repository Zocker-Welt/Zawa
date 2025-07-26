#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn helloworld() {
        println!("Hello, world!");
    }

    #[test]
    fn interpret_helloworld() {
        let output = Command::new("cargo")
            .args(["r", "C:/Users/Misha/Documents/zawa/src/tests/cases/helloworld.zw"])
            .output()
            .unwrap();

        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "Hello, world!")
    }

    #[test]
    fn interpret_block() {
        let output = Command::new("cargo")
            .args(["r", "C:/Users/Misha/Documents/zawa/src/tests/cases/block.zw"])
            .output()
            .unwrap();

        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "3");
        assert_eq!(lines[1], "3");

    }

    #[test]
    fn interpret_while() {
        let output = Command::new("cargo")
            .args(["r", "C:/Users/Misha/Documents/zawa/src/tests/cases/while.zw"])
            .output()
            .unwrap();

        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        assert_eq!(lines.len(), 6);
        assert_eq!(lines[0], "4");
        assert_eq!(lines[1], "3");
        assert_eq!(lines[2], "2");
        assert_eq!(lines[3], "1");
        assert_eq!(lines[4], "0");
    }

    #[test]
    fn interpret_for_0_8() {
        let output = Command::new("cargo")
            .args(["r", "C:/Users/Misha/Documents/zawa/src/tests/cases/for_0_8.zw"])
            .output()
            .unwrap();

        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        assert_eq!(lines.len(), 6);
        assert_eq!(lines[0], "0");
        assert_eq!(lines[1], "1");
        assert_eq!(lines[2], "2");
        assert_eq!(lines[3], "3");
        assert_eq!(lines[4], "4");
    }

    #[test]
    fn interpret_for() {
        let output = Command::new("cargo")
            .args(["r", "C:/Users/Misha/Documents/zawa/src/tests/cases/for.zw"])
            .output()
            .unwrap();

        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

            assert_eq!(lines.len(), 13);
            assert_eq!(lines[0], "0");
            assert_eq!(lines[1], "1");
            assert_eq!(lines[2], "1");
            assert_eq!(lines[3], "2");
            assert_eq!(lines[4], "3");
            assert_eq!(lines[5], "5");
            assert_eq!(lines[6], "8");
            assert_eq!(lines[7], "13");
            assert_eq!(lines[8], "21");
            assert_eq!(lines[9], "34");
            assert_eq!(lines[10], "55");
            assert_eq!(lines[11], "89");
    }

    #[test]
    fn interpret_break() {
        let output = Command::new("cargo")
            .args(["r", "C:/Users/Misha/Documents/zawa/src/tests/cases/break.zw"])
            .output()
            .unwrap();

        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        assert_eq!(lines.len(), 8);
        assert_eq!(lines[0], "0 1 2 3 4 5 ");
        assert_eq!(lines[1], "0 1 2 3 4 5 ");
        assert_eq!(lines[2], "0 1 2 3 4 5 ");
        assert_eq!(lines[3], "0 1 2 3 4 5 ");
        assert_eq!(lines[4], "0 1 2 3 4 5 ");
        assert_eq!(lines[5], "0 1 2 3 4 5 ");
        assert_eq!(lines[6], "That's correct!");
    }

    #[test]
    fn interpret_recursive_count() {
        let output = Command::new("cargo")
            .args(["r", "C:/Users/Misha/Documents/zawa/src/tests/cases/recursive_count.zw"])
            .output()
            .unwrap();

        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        assert_eq!(lines.len(), 4);
        assert_eq!(lines[0], "1");
        assert_eq!(lines[1], "2");
        assert_eq!(lines[2], "3");
    }

    #[test]
    fn interpret_fn_modify_local_env() {
        let output = Command::new("cargo")
            .args(["r", "C:/Users/Misha/Documents/zawa/src/tests/cases/fn_modify_local_env.zw"])
            .output()
            .unwrap();

        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "5");
    }

    #[test]
    fn interpret_fn_return() {
        let output = Command::new("cargo")
            .args(["r", "C:/Users/Misha/Documents/zawa/src/tests/cases/fn_return.zw"])
            .output()
            .unwrap();

        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "12");
    }

    #[test]
    fn interpret_fn_return_null() {
        let output = Command::new("cargo")
            .args(["r", "C:/Users/Misha/Documents/zawa/src/tests/cases/fn_return_null.zw"])
            .output()
            .unwrap();

        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "12");
        assert_eq!(lines[1], "null");
    }

    #[test]
    fn interpret_fn_cond_return() {
        let output = Command::new("cargo")
            .args(["r", "C:/Users/Misha/Documents/zawa/src/tests/cases/fn_cond_return.zw"])
            .output()
            .unwrap();

        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        assert_eq!(lines.len(), 4);
        assert_eq!(lines[0], "1");
        assert_eq!(lines[1], "0");
        assert_eq!(lines[2], "1");
    }

    #[test]
    fn interpret_fn_nested_return() {
        let output = Command::new("cargo")
            .args(["r", "C:/Users/Misha/Documents/zawa/src/tests/cases/fn_nested_return.zw"])
            .output()
            .unwrap();

        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        assert_eq!(lines.len(), 4);
        assert_eq!(lines[0], "7");
        assert_eq!(lines[1], "3");
        assert_eq!(lines[2], "4");
    }

    #[test]
    fn interpret_fn_fib() {
        let output = Command::new("cargo")
            .args(["r", "C:/Users/Misha/Documents/zawa/src/tests/cases/fn_fib.zw"])
            .output()
            .unwrap();

        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        let ans = &["0", "1", "1", "2", "3", "5", "8", "13", "21", "34", "55", "89", "144", "233", "377", "610", "987", "1597", "2584", "4181"];
        assert_eq!(lines.len(), 21);
        for i in 0..(lines.len() - 1) {
            assert_eq!(lines[i], ans[i]);
        }
    }

    #[test]
    fn interpret_fn_closure() {
        let output = Command::new("cargo")
            .args(["r", "C:/Users/Misha/Documents/zawa/src/tests/cases/fn_closure.zw"])
            .output()
            .unwrap();

        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        let ans = &["1", "1", "2", "2"];
        assert_eq!(lines.len(), ans.len() + 1);
        for i in 0..(lines.len() - 1) {
            assert_eq!(lines[i], ans[i]);
        }
    }

    #[test]
    fn interpret_fn_anon() {
        let output = Command::new("cargo")
            .args(["r", "C:/Users/Misha/Documents/zawa/src/tests/cases/fn_anon.zw"])
            .output()
            .unwrap();

        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        let ans = &["0", "1", "2"];
        assert_eq!(lines.len(), ans.len() + 1);
        for i in 0..(lines.len() - 1) {
            assert_eq!(lines[i], ans[i]);
        }
    }

    fn interpret_fn_anon_2() {
        let output = Command::new("cargo")
            .args(["r", "C:/Users/Misha/Documents/zawa/src/tests/cases/fn_anon_2.zw"])
            .output()
            .unwrap();

        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        let ans = &["1"];
        assert_eq!(lines.len(), ans.len() + 1);
        for i in 0..(lines.len() - 1) {
            assert_eq!(lines[i], ans[i]);
        }
    }

    fn interpret_fn_anon_declaration() {
        let output = Command::new("cargo")
            .args(["r", "C:/Users/Misha/Documents/zawa/src/tests/cases/fn_anon_declaration.zw"])
            .output()
            .unwrap();

        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        let ans = &["0"];
        assert_eq!(lines.len(), ans.len() + 1);
        for i in 0..(lines.len() - 1) {
            assert_eq!(lines[i], ans[i]);
        }
    }
}