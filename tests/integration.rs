extern crate assert_cli;

#[cfg(test)]
mod integration {
    use assert_cli;

    #[test]
    fn calling_rget_without_args() {
        assert_cli::Assert::main_binary()
            .stdout()
            .contains("You are going to listen to")
            .stdout()
            .contains("Here's the url:")
            .unwrap();
    }
}
