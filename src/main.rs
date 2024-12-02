pub mod utils;
mod days;

fn main() {
    #[cfg(feature="day1")]
    days::day1::star1::solve();
    #[cfg(feature="day2")]
    days::day1::star1::solve();

}
