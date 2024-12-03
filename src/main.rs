pub mod utils;
mod days;

fn main() {
    #[cfg(feature="day1")]
    days::day1::star1::solve();
    #[cfg(feature="day1")]
    days::day1::star2::solve();
    
    #[cfg(feature="day2")]
    days::day2::star1::solve();
    #[cfg(feature="day2")]
    days::day2::star2::solve();

    #[cfg(feature="day3")]
    days::day3::star1::solve();
    #[cfg(feature="day3")]
    days::day3::star2::solve();
}
