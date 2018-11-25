extern crate rand;
extern crate rspotify;

mod user;

use user::get_followed_artists;

fn main() {
    get_followed_artists();
}
