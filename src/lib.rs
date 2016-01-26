/*
Copyright ⓒ 2016 Daniel Keep.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
/*!

Provides the [`Boolinator`](trait.Boolinator.html) trait, which lets you use `Option` and `Result`-style combinators with `bool`s.

<style type="text/css">
.link-block { font-family: "Fira Sans"; }
.link-block > p { display: inline-block; }
.link-block > p > strong { font-weight: 500; margin-right: 1em; }
.link-block > ul { display: inline-block; padding: 0; list-style: none; }
.link-block > ul > li {
  font-size: 0.8em;
  background-color: #eee;
  border: 1px solid #ccc;
  padding: 0.3em;
  display: inline-block;
}
</style>
<span></span><div class="link-block">

**Links**

* [Latest Release](https://crates.io/crates/boolinator/)
* [Latest Docs](https://danielkeep.github.io/rust-boolinator/doc/boolinator/index.html)
* [Repository](https://github.com/DanielKeep/rust-boolinator)

<span></span></div>

## Compatibility

v0.1.0 was tested against `rustc` versions 1.0.0, 1.1.0, 1.2.0, 1.3.0, 1.4.0, 1.5.0, 1.6.0, 1.7.0-beta.1, and nightly 2016-01-20.

*/
// Can't have undocumented APIs!  Nosiree!
#![deny(missing_docs)]

/**
This trait defines a number of combinator-style methods for use with `bool` values.

In general, `true`/`false` map to `Some(_)`/`None` and `Ok(_)`/`Err(_)` respectively.
*/
pub trait Boolinator: Sized {
    /**
    Converts this value into a logically equivalent `Some(())` or `None`.
    */
    fn as_option(self) -> Option<()>;

    /**
    If this value is `true`, returns `Some(some)`; `None` otherwise.
    */
    fn as_some<T>(self, some: T) -> Option<T>;

    /**
    If this value is `true`, returns `Some(some())`; `None` otherwise. 
    */
    fn as_some_from<T, F>(self, some: F) -> Option<T>
    where F: FnOnce() -> T;

    /**
    If this value is `true`, returns `opt`; `None` otherwise. 
    */
    fn and_option<T>(self, opt: Option<T>) -> Option<T>;

    /**
    If this value is `true`, returns `opt()`; `None` otherwise. 
    */
    fn and_option_from<T, F>(self, opt: F) -> Option<T>
    where F: FnOnce() -> Option<T>;

    /**
    If this value is `true`, returns `Ok(ok)`; `Err(err)` otherwise. 
    */
    fn as_result<T, E>(self, ok: T, err: E) -> Result<T, E>;

    /**
    If this value is `true`, returns `Ok(ok())`; `Err(err())` otherwise. 
    */
    fn as_result_from<T, E, F, G>(self, ok: F, err: G) -> Result<T, E>
    where F: FnOnce() -> T, G: FnOnce() -> E;

    /**
    Panics with `msg` if this value is `false`, otherwise it does nothing.
    */
    fn expect(self, msg: &str);
}

impl Boolinator for bool {
    #[inline]
    fn as_option(self) -> Option<()> {
        if self { Some(()) } else { None }
    }

    fn as_some<T>(self, some: T) -> Option<T> {
        if self { Some(some) } else { None }
    }

    fn as_some_from<T, F>(self, some: F) -> Option<T>
    where F: FnOnce() -> T {
        if self { Some(some()) } else { None }
    }

    fn and_option<T>(self, opt: Option<T>) -> Option<T> {
        if self { opt } else { None }
    }

    fn and_option_from<T, F>(self, opt: F) -> Option<T>
    where F: FnOnce() -> Option<T> {
        if self { opt() } else { None }
    }

    fn as_result<T, E>(self, ok: T, err: E) -> Result<T, E> {
        if self { Ok(ok) } else { Err(err) }
    }

    fn as_result_from<T, E, F, G>(self, ok: F, err: G) -> Result<T, E>
    where F: FnOnce() -> T, G: FnOnce() -> E {
        if self { Ok(ok()) } else { Err(err()) }
    }
    
    #[inline]
    fn expect(self, msg: &str) {
        if self { () } else { panic!("{}", msg) }
    }
}

/*
Serious code must have serious tests, and Boolinator is serious business!
*/
#[cfg(test)]
mod tests {
    use super::Boolinator; // as opposed to the original NES version.

    #[test]
    fn test_as_option() {
        // Very test.
        assert_eq!(true.as_option(), Some(()));
        assert_eq!(false.as_option(), None);
    }

    #[test]
    fn test_as_some() {
        // Much serious.
        let love = true;
        let everybody = love.as_some("body").expect("needs");
        assert_eq!(everybody, "body");

        assert_eq!((!love).as_some("money can buy"), None);
    }

    #[test]
    fn test_as_some_from() {
        // Wow.
        let mothers = vec![true, false, false, true, false, true];
        assert!(mothers.into_iter()
            .map(|e| e.as_some_from(|| Some("em")))
            .filter(Option::is_some)
            .count() > 0);
    }

    #[test]
    fn test_and_option() {
        // Such original.
        assert_eq!(true.and_option(Some("fries with that")), Some("fries with that"));
        assert_eq!(false.and_option(Some("fries with that")), None);
        assert_eq!(true.and_option(None), None::<()>);
        assert_eq!(false.and_option(None), None::<()>);
    }

    #[test]
    fn test_and_option_from() {
        // Such original.
        assert_eq!(true.and_option_from(|| Some("chips too, guv'")), Some("chips too, guv'"));
        assert_eq!(false.and_option_from(|| Some("chips too, guv'")), None);
        assert_eq!(true.and_option_from(|| None), None::<()>);
        assert_eq!(false.and_option_from(|| None), None::<()>);
    }

    #[test]
    fn test_as_result() {
        // Very result.
        assert_eq!(true.as_result("now; ", ", what?"),   Ok("now; "));
        assert_eq!(false.as_result("now; ", ", what?"),  Err(", what?"));
    }

    #[test]
    fn test_as_result_from() {
        // Code good.
        assert_eq!(true.as_result_from(|| "four space indent", || "anything else"), Ok("four space indent"));
        assert_eq!(false.as_result_from(|| "four space indent", || "anything else"), Err("anything else"));
    }

    const DREAMS: &'static str = "love and financial security";

    #[test]
    fn test_expect() {
        // Movies lie.
        true.expect(DREAMS);
    }

    #[test]
    #[should_panic]
    fn test_expect_reality() {
        // Send hugs.
        false.expect(DREAMS);
    }
}