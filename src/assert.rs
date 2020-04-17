/*
 * Copyright (2020) by Marcel Lambert.
 * This project's License is the MIT Open Source license.
 * For more information, see the LICENSE.md file in this repository.
 */

//! utilitys for testing.

#[cfg(test)]
pub(crate) mod tests {

    /// Useful to test if a type implements `Sync`.
    ///
    /// # Example
    ///
    /// ```
    /// #[test]
    /// fn test_send() {
    ///     fn assert_send<T: Send>() {}
    ///     assert_send::<MyStrangeType>();
    /// }
    /// ```
    ///
    /// Use it to check if the compiler is able to auto derive `Sync`.
    ///
    /// # Copyright
    /// shamelessly copied from [Api Guidelines](https://rust-lang.github.io/api-guidelines/interoperability.html#types-are-send-and-sync-where-possible-c-send-sync)
    pub(crate) fn assert_sync<T: Sync>() {}

    /// Useful to test if a type implements `Send`.
    ///
    /// # Example
    ///
    /// ```
    /// #[test]
    /// fn test_send() {
    ///     fn assert_send<T: Send>() {}
    ///     assert_send::<MyStrangeType>();
    /// }
    /// ```
    ///
    /// Use it to check if the compiler is able to auto derive `Send`.
    ///
    /// # Copyright
    /// shamelessly copied from [Api Guidelines](https://rust-lang.github.io/api-guidelines/interoperability.html#types-are-send-and-sync-where-possible-c-send-sync)
    pub(crate) fn assert_send<T: Send>() {}

    #[test]
    fn test_sync() {
        struct ShouldBeSyncable {};
        assert_sync::<ShouldBeSyncable>();
    }

    #[test]
    fn test_send() {
        struct ShouldBeSendable {};
        assert_send::<ShouldBeSendable>();
    }
}
