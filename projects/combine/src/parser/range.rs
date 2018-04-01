//! Module containing zero-copy parsers.

use lib::marker::PhantomData;

use Parser;
use stream::{uncons_range, uncons_while, wrap_stream_error, RangeStream, RangeStreamOnce,
             Resetable, StreamOnce};
use error::{ConsumedResult, Info, ParseError, Tracked};
use error::FastResult::*;
use parser::ParseMode;

pub struct Range<I>(I::Range)
where
    I: RangeStream;

impl<I> Parser for Range<I>
where
    I: RangeStream,
    I::Range: PartialEq + ::stream::Range,
{
    type Input = I;
    type Output = I::Range;
    type PartialState = ();

    #[inline]
    fn parse_lazy(&mut self, input: &mut Self::Input) -> ConsumedResult<Self::Output, Self::Input> {
        use stream::Range;
        let position = input.position();
        match input.uncons_range(self.0.len()) {
            Ok(other) => if other == self.0 {
                ConsumedOk(other)
            } else {
                EmptyErr(I::Error::empty(position).into())
            },
            Err(err) => wrap_stream_error(input, err),
        }
    }
    fn add_error(&mut self, errors: &mut Tracked<<Self::Input as StreamOnce>::Error>) {
        // TODO Add unexpected message?
        errors.error.add_expected(Info::Range(self.0.clone()));
    }
}

parser!{
    #[derive(Clone)]
    pub struct Recognize;
    /// Zero-copy parser which returns consumed input range.
    ///
    /// ```
    /// # extern crate combine;
    /// # use combine::parser::range::recognize;
    /// # use combine::parser::char::letter;
    /// # use combine::*;
    /// # fn main() {
    /// let mut parser = recognize(skip_many1(letter()));
    /// assert_eq!(parser.parse("hello world"), Ok(("hello", " world")));
    /// assert!(parser.parse("!").is_err());
    /// # }
    /// ```
    #[inline(always)]
    pub fn recognize[P](parser: P)(P::Input) -> <P::Input as StreamOnce>::Range
    where [
        P: Parser,
        P::Input: RangeStream,
        <P::Input as StreamOnce>::Range: ::stream::Range,
    ]
    {
        recognize_with_value(parser).map(|(range, _)| range)
    }
}

#[inline]
fn parse_partial_range<M, F, I>(
    mode: M,
    input: &mut I,
    distance_state: &mut usize,
    f: F,
) -> ConsumedResult<I::Range, I>
where
    M: ParseMode,
    F: FnOnce(&mut I) -> ConsumedResult<I::Range, I>,
    I: RangeStream,
    I::Range: ::stream::Range,
{
    let before = input.checkpoint();

    if mode.is_first() || *distance_state == 0 {
        let result = f(input);
        if let ConsumedErr(_) = result {
            *distance_state = input.distance(&before);
            input.reset(before);
        }
        result
    } else {
        if let Err(_) = input.uncons_range(*distance_state) {
            panic!("recognize errored when restoring the input stream to its expected state");
        }

        match f(input) {
            ConsumedOk(_) | EmptyOk(_) => (),
            EmptyErr(err) => return EmptyErr(err),
            ConsumedErr(err) => {
                *distance_state = input.distance(&before);
                input.reset(before);
                return ConsumedErr(err);
            }
        }

        let distance = input.distance(&before);
        input.reset(before);
        take(distance).parse_lazy(input).map(|range| {
            *distance_state = 0;
            range
        })
    }
}

#[derive(Clone)]
pub struct RecognizeWithValue<P>(P);

impl<P> Parser for RecognizeWithValue<P>
where
    P: Parser,
    P::Input: RangeStream,
    <P::Input as StreamOnce>::Range: ::stream::Range,
{
    type Input = P::Input;
    type Output = (<P::Input as StreamOnce>::Range, P::Output);
    type PartialState = (usize, P::PartialState);

    #[inline]
    fn parse_partial(
        &mut self,
        input: &mut Self::Input,
        state: &mut Self::PartialState,
    ) -> ConsumedResult<Self::Output, Self::Input> {
        let (ref mut distance_state, ref mut child_state) = *state;

        let before = input.checkpoint();
        if let Err(_) = input.uncons_range(*distance_state) {
            panic!("recognize errored when restoring the input stream to its expected state");
        }

        let value = match self.0.parse_partial(input, child_state) {
            ConsumedOk(x) | EmptyOk(x) => x,
            EmptyErr(err) => return EmptyErr(err),
            ConsumedErr(err) => {
                *distance_state = input.distance(&before);
                input.reset(before);
                return ConsumedErr(err);
            }
        };

        let distance = input.distance(&before);
        input.reset(before);
        take(distance).parse_lazy(input).map(|range| {
            *distance_state = 0;
            (range, value)
        })
    }
    fn add_error(&mut self, errors: &mut Tracked<<Self::Input as StreamOnce>::Error>) {
        self.0.add_error(errors)
    }
}

/// Zero-copy parser which returns a pair: (consumed input range, parsed value).
///
/// ```
/// # extern crate combine;
/// # use combine::parser::range::recognize_with_value;
/// # use combine::parser::char::{digit, char};
/// # use combine::*;
/// # fn main() {
/// let mut parser = recognize_with_value((
///     skip_many1(digit()),
///     optional((try(char('.')), skip_many1(digit()))),
/// ).map(|(_, opt)| opt.is_some()));
///
/// assert_eq!(parser.parse("1234!"), Ok((("1234", false), "!")));
/// assert_eq!(parser.parse("1234.0001!"), Ok((("1234.0001", true), "!")));
/// assert!(parser.parse("!").is_err());
/// assert!(parser.parse("1234.").is_err());
/// # }
/// ```
#[inline(always)]
pub fn recognize_with_value<P>(parser: P) -> RecognizeWithValue<P>
where
    P: Parser,
    P::Input: RangeStream,
    <P::Input as StreamOnce>::Range: ::stream::Range,
{
    RecognizeWithValue(parser)
}

/// Zero-copy parser which reads a range of length `i.len()` and succeeds if `i` is equal to that
/// range.
///
/// ```
/// # extern crate combine;
/// # use combine::parser::range::range;
/// # use combine::*;
/// # fn main() {
/// let mut parser = range("hello");
/// let result = parser.parse("hello world");
/// assert_eq!(result, Ok(("hello", " world")));
/// let result = parser.parse("hel world");
/// assert!(result.is_err());
/// # }
/// ```
#[inline(always)]
pub fn range<I>(i: I::Range) -> Range<I>
where
    I: RangeStream,
    I::Range: PartialEq + ::stream::Range,
{
    Range(i)
}

pub struct Take<I>(usize, PhantomData<fn(I) -> I>);
impl<I> Parser for Take<I>
where
    I: RangeStream,
    I::Range: ::stream::Range,
{
    type Input = I;
    type Output = I::Range;
    type PartialState = ();

    #[inline]
    fn parse_lazy(&mut self, input: &mut Self::Input) -> ConsumedResult<Self::Output, Self::Input> {
        uncons_range(input, self.0)
    }
}

/// Zero-copy parser which reads a range of length `n`.
///
/// ```
/// # extern crate combine;
/// # use combine::parser::range::take;
/// # use combine::*;
/// # fn main() {
/// let mut parser = take(1);
/// let result = parser.parse("1");
/// assert_eq!(result, Ok(("1", "")));
/// let mut parser = take(4);
/// let result = parser.parse("123abc");
/// assert_eq!(result, Ok(("123a", "bc")));
/// let result = parser.parse("abc");
/// assert!(result.is_err());
/// # }
/// ```
#[inline(always)]
pub fn take<I>(n: usize) -> Take<I>
where
    I: RangeStream,
    I::Range: ::stream::Range,
{
    Take(n, PhantomData)
}

pub struct TakeWhile<I, F>(F, PhantomData<fn(I) -> I>);
impl<I, F> Parser for TakeWhile<I, F>
where
    I: RangeStream,
    I::Range: ::stream::Range,
    F: FnMut(I::Item) -> bool,
{
    type Input = I;
    type Output = I::Range;
    type PartialState = usize;

    parse_mode!();
    #[inline]
    fn parse_mode_impl<M>(
        &mut self,
        mode: M,
        input: &mut Self::Input,
        state: &mut Self::PartialState,
    ) -> ConsumedResult<Self::Output, Self::Input>
    where
        M: ParseMode,
    {
        parse_partial_range(mode, input, state, |input| uncons_while(input, &mut self.0))
    }
}

/// Zero-copy parser which reads a range of 0 or more tokens which satisfy `f`.
///
/// ```
/// # extern crate combine;
/// # use combine::parser::range::take_while;
/// # use combine::*;
/// # fn main() {
/// let mut parser = take_while(|c: char| c.is_digit(10));
/// let result = parser.parse("123abc");
/// assert_eq!(result, Ok(("123", "abc")));
/// let result = parser.parse("abc");
/// assert_eq!(result, Ok(("", "abc")));
/// # }
/// ```
#[inline(always)]
pub fn take_while<I, F>(f: F) -> TakeWhile<I, F>
where
    I: RangeStream,
    F: FnMut(I::Item) -> bool,
{
    TakeWhile(f, PhantomData)
}

pub struct TakeWhile1<I, F>(F, PhantomData<fn(I) -> I>);
impl<I, F> Parser for TakeWhile1<I, F>
where
    I: RangeStream,
    I::Range: ::stream::Range,
    F: FnMut(I::Item) -> bool,
{
    type Input = I;
    type Output = I::Range;
    type PartialState = usize;

    parse_mode!();
    #[inline]
    fn parse_mode_impl<M>(
        &mut self,
        mode: M,
        input: &mut Self::Input,
        state: &mut Self::PartialState,
    ) -> ConsumedResult<Self::Output, Self::Input>
    where
        M: ParseMode,
    {
        let start = input.position();
        parse_partial_range(mode, input, state, |input| {
            let result = uncons_while(input, &mut self.0);
            let position = input.position();
            if start == position {
                if let EmptyOk(_) = result {
                    EmptyErr(I::Error::empty(position).into())
                } else {
                    result
                }
            } else {
                result
            }
        })
    }
}

/// Zero-copy parser which reads a range of 1 or more tokens which satisfy `f`.
///
/// ```
/// # extern crate combine;
/// # use combine::parser::range::take_while1;
/// # use combine::*;
/// # fn main() {
/// let mut parser = take_while1(|c: char| c.is_digit(10));
/// let result = parser.parse("123abc");
/// assert_eq!(result, Ok(("123", "abc")));
/// let result = parser.parse("abc");
/// assert!(result.is_err());
/// # }
/// ```
#[inline(always)]
pub fn take_while1<I, F>(f: F) -> TakeWhile1<I, F>
where
    I: RangeStream,
    I::Range: ::stream::Range,
    F: FnMut(I::Item) -> bool,
{
    TakeWhile1(f, PhantomData)
}

pub struct TakeUntilRange<I>(I::Range)
where
    I: RangeStream;
impl<I> Parser for TakeUntilRange<I>
where
    I: RangeStream,
    I::Range: PartialEq + ::stream::Range,
{
    type Input = I;
    type Output = I::Range;
    type PartialState = usize;

    #[inline]
    fn parse_partial(
        &mut self,
        input: &mut Self::Input,
        to_consume: &mut Self::PartialState,
    ) -> ConsumedResult<Self::Output, Self::Input> {
        use stream::Range;

        let len = self.0.len();
        let before = input.checkpoint();

        // Skip until the end of the last parse attempt
        ctry!(uncons_range(input, *to_consume));

        loop {
            let look_ahead_input = input.checkpoint();

            match input.uncons_range(len) {
                Ok(xs) => {
                    if xs == self.0 {
                        input.reset(before);
                        if let Ok(consumed) = input.uncons_range(*to_consume) {
                            if *to_consume == 0 {
                                return EmptyOk(consumed);
                            } else {
                                *to_consume = 0;
                                return ConsumedOk(consumed);
                            }
                        }

                        // We are guaranteed able to uncons to_consume characters here
                        // because we've already done it on look_ahead_input.
                        unreachable!();
                    } else {
                        input.reset(look_ahead_input);
                        *to_consume += 1;
                        if input.uncons().is_err() {
                            unreachable!();
                        }
                    }
                }
                Err(e) => {
                    input.reset(before);
                    return wrap_stream_error(input, e);
                }
            };
        }
    }
}

/// Zero-copy parser which reads a range of 0 or more tokens until `r` is found.
///
/// The range `r` will not be consumed. If `r` is not found, the parser will
/// return an error.
///
/// ```
/// # extern crate combine;
/// # use combine::parser::range::{range, take_until_range};
/// # use combine::*;
/// # fn main() {
/// let mut parser = take_until_range("\r\n");
/// let result = parser.parse("To: user@example.com\r\n");
/// assert_eq!(result, Ok(("To: user@example.com", "\r\n")));
/// let result = parser.parse("Hello, world\n");
/// assert!(result.is_err());
/// # }
/// ```
#[inline(always)]
pub fn take_until_range<I>(r: I::Range) -> TakeUntilRange<I>
where
    I: RangeStream,
{
    TakeUntilRange(r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use Parser;

    #[test]
    fn take_while_test() {
        let result = take_while(|c: char| c.is_digit(10)).parse("123abc");
        assert_eq!(result, Ok(("123", "abc")));
        let result = take_while(|c: char| c.is_digit(10)).parse("abc");
        assert_eq!(result, Ok(("", "abc")));
    }

    #[test]
    fn take_while1_test() {
        let result = take_while1(|c: char| c.is_digit(10)).parse("123abc");
        assert_eq!(result, Ok(("123", "abc")));
        let result = take_while1(|c: char| c.is_digit(10)).parse("abc");
        assert!(result.is_err());
    }

    #[test]
    fn range_string_no_char_boundary_error() {
        let mut parser = range("hello");
        let result = parser.parse("hell\u{00EE} world");
        assert!(result.is_err());
    }
}
