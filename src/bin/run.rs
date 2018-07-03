extern crate notmuch_rs;

use notmuch_rs::{Notmuch, NotmuchCommand, NotmuchOutputType};

fn main() {
    let output = Notmuch::new(NotmuchCommand::Search, NotmuchOutputType::Summary)
        .arg("tag:personal")
        .arg("from:chris@thesogu.com")
        .exec();

    if let Ok(data) = output {
        println!("{:?}", data);
    } else {
        println!("shit what the fuck happened")
    }
}
