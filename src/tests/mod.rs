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
}