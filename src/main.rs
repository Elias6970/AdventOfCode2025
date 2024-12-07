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

    #[cfg(feature="day4")]
    days::day4::star1::solve();
    #[cfg(feature="day4")]
    days::day4::star2::solve();

    #[cfg(feature="day5")]
    days::day5::star1::solve();
    #[cfg(feature="day5")]
    days::day5::star2::solve();

    #[cfg(feature="day6")]
    days::day6::star1::solve();
    #[cfg(feature="day6")]
    days::day6::star2::solve();

    #[cfg(feature="day7")]
    days::day7::star1::solve();
    
}
