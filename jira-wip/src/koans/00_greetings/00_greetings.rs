/// **Practicalities**:
/// - each koan is a sub-folder in the `koans` folder;
/// - each folder contains multiple test files with one or more tests in each of it;
/// - you can move along your journey with `koans --path jira-wip`:
///     - if you have successfully completed an exercise (or you just started this workshop)
///       the console output will ask if you want to move forward and
///       tell you the name of the next koan you should get started with if you say "yes";
///     - if something is wrong with your test cases, the console output will contain
///       the compiler errors or test failures that you should investigate.
///
/// ~ Enjoy! ~

#[cfg(test)]
mod tests {
    #[test]
    /// This is your starting block!
    ///
    /// In each test, you are expected to replace __ or todo!() in order to make test pass.
    fn the_beginning_of_your_journey() {
        let i_am_ready_to_start = true;

        assert!(i_am_ready_to_start);
    }
}
