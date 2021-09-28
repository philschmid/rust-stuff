use chrono::{Utc};

use chrono::prelude::*;


fn main() {
    let now = Utc::now();
    let dt = Utc.ymd(2021,10, 14).and_hms(23, 59, 59);

    if (now > dt) {
       return 
    }
    println!("{}", dt);

}