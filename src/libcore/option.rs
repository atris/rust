/*!
 * Operations on the ubiquitous `option` type.
 *
 * Type `option` represents an optional value.
 *
 * Every `option<T>` value can either be `some(T)` or `none`. Where in other
 * languages you might use a nullable type, in Rust you would use an option
 * type.
 */

/// The option type
enum option<T> {
    none,
    some(T),
}

pure fn get<T: copy>(opt: option<T>) -> T {
    /*!
     * Gets the value out of an option
     *
     * # Failure
     *
     * Fails if the value equals `none`
     */

    alt opt { some(x) { ret x; } none { fail "option none"; } }
}

pure fn expect<T: copy>(opt: option<T>, reason: str) -> T {
    #[doc = "
    Gets the value out of an option, printing a specified message on failure

    # Failure

    Fails if the value equals `none`
    "];
    alt opt { some(x) { x } none { fail reason; } }
}

pure fn map<T, U: copy>(opt: option<T>, f: fn(T) -> U) -> option<U> {
    //! Maps a `some` value from one type to another

    alt opt { some(x) { some(f(x)) } none { none } }
}

pure fn chain<T, U>(opt: option<T>, f: fn(T) -> option<U>) -> option<U> {
    /*!
     * Update an optional value by optionally running its content through a
     * function that returns an option.
     */

    alt opt { some(x) { f(x) } none { none } }
}

pure fn is_none<T>(opt: option<T>) -> bool {
    //! Returns true if the option equals `none`

    alt opt { none { true } some(_) { false } }
}

pure fn is_some<T>(opt: option<T>) -> bool {
    //! Returns true if the option contains some value

    !is_none(opt)
}

pure fn get_default<T: copy>(opt: option<T>, def: T) -> T {
    //! Returns the contained value or a default

    alt opt { some(x) { x } none { def } }
}

pure fn map_default<T, U: copy>(opt: option<T>, def: U, f: fn(T) -> U) -> U {
    //! Applies a function to the contained value or returns a default

    alt opt { none { def } some(t) { f(t) } }
}

pure fn iter<T>(opt: option<T>, f: fn(T)) {
    //! Performs an operation on the contained value or does nothing

    alt opt { none { } some(t) { f(t); } }
}

pure fn unwrap<T>(-opt: option<T>) -> T {
    /*!
     * Moves a value out of an option type and returns it.
     *
     * Useful primarily for getting strings, vectors and unique pointers out
     * of option types without copying them.
     */

    unsafe {
        let addr = alt opt {
          some(x) { ptr::addr_of(x) }
          none { fail "option none" }
        };
        let liberated_value = unsafe::reinterpret_cast(*addr);
        unsafe::forget(opt);
        ret liberated_value;
    }
}

impl extensions<T> for option<T> {
    /**
     * Update an optional value by optionally running its content through a
     * function that returns an option.
     */
    fn chain<U>(f: fn(T) -> option<U>) -> option<U> { chain(self, f) }
    /// Applies a function to the contained value or returns a default
    fn map_default<U: copy>(def: U, f: fn(T) -> U) -> U
        { map_default(self, def, f) }
    /// Performs an operation on the contained value or does nothing
    fn iter(f: fn(T)) { iter(self, f) }
    /// Returns true if the option equals `none`
    fn is_none() -> bool { is_none(self) }
    /// Returns true if the option contains some value
    fn is_some() -> bool { is_some(self) }
    /// Maps a `some` value from one type to another
    fn map<U:copy>(f: fn(T) -> U) -> option<U> { map(self, f) }
}

impl extensions<T: copy> for option<T> {
    /**
     * Gets the value out of an option
     *
     * # Failure
     *
     * Fails if the value equals `none`
     */
    fn get() -> T { get(self) }
    fn get_default(def: T) -> T { get_default(self, def) }
    /**
     * Gets the value out of an option, printing a specified message on
     * failure
     *
     * # Failure
     *
     * Fails if the value equals `none`
     */
    pure fn expect(reason: str) -> T { expect(self, reason) }
}

#[test]
fn test_unwrap_ptr() {
    let x = ~0;
    let addr_x = ptr::addr_of(*x);
    let opt = some(x);
    let y = unwrap(opt);
    let addr_y = ptr::addr_of(*y);
    assert addr_x == addr_y;
}

#[test]
fn test_unwrap_str() {
    let x = "test";
    let addr_x = str::as_buf(x, |buf| ptr::addr_of(buf));
    let opt = some(x);
    let y = unwrap(opt);
    let addr_y = str::as_buf(y, |buf| ptr::addr_of(buf));
    assert addr_x == addr_y;
}

#[test]
fn test_unwrap_resource() {
    class r {
       let i: @mut int;
       new(i: @mut int) { self.i = i; }
       drop { *(self.i) += 1; }
    }
    let i = @mut 0;
    {
        let x = r(i);
        let opt = some(x);
        let _y = unwrap(opt);
    }
    assert *i == 1;
}

// Local Variables:
// mode: rust;
// fill-column: 78;
// indent-tabs-mode: nil
// c-basic-offset: 4
// buffer-file-coding-system: utf-8-unix
// End:
