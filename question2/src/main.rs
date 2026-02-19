pub struct L{
    x: usize,
    y: usize,
}

pub fn foo(text: &str, string: &str) -> Vec<L> {
    text.lines().collect::<Vec<&str>>().iter().enumerate()
    .map(|(x, line)| line.match_indices(string).map(move |(y, _)| L{x, y})).flatten().collect()
}

fn main() {
    let results = foo("Shall I compare thee to a summer's day?
    Thou art more lovely and more temperate:
    Rough winds do shake the darling buds of May,
    And summer's lease hath all too short a date:
    Sometimes too hot the eye of heaven shines,
    And too often is his gold complexion dimm'd:
    And every fair from fair sometimes declines,
    By chance or natures changing course untrimm'd;
    By thy eternal summer shall not fade,
    Nor lose possession of that fair thou owest;
    Nor shall Death brag thou wander'st in his shade,
    When in eternal lines to time thou growest:
    So long as men can breathe or eyes can see,
    So long lives this and this gives life to thee.", "the");

    for x in results {println!("x : {}, y : {}", x.x, x.y);}

}

// Output:
// x : 0, y : 16
// x : 2, y : 25
// x : 4, y : 22
// x : 12, y : 27
// x : 13, y : 46

// References:
// https://users.rust-lang.org/t/flattening-and-collecting-a-nested-iterator-of-results/68525
// https://stackoverflow.com/questions/58737024/how-to-get-the-index-of-the-current-element-being-processed-in-the-iteration-wit
// https://doc.rust-lang.org/beta/std/iter/struct.Flatten.html
// https://stackoverflow.com/questions/67277966/rust-no-method-named-iter-found-for-struct-stdopsrangeusize-in-the-cu
// https://users.rust-lang.org/t/where-is-flatten-skipping-none-documented/89255
// https://users.rust-lang.org/t/flatten-an-iterator-of-result-vec-t-error-while-collecting-it/63766
