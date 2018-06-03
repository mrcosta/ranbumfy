extern crate rspotify;

mod user_followed_artists;

fn main() {
    user_followed_artists::get_followed_artists();
}
