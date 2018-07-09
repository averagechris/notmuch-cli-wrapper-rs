extern crate notmuch_rs;

use notmuch_rs::search::Search;
use notmuch_rs::search::SearchOutputType::Threads;
use notmuch_rs::show::Show;
use notmuch_rs::NotmuchCommand;

fn do_search() {
    let command = Search::new()
        .with_tags(vec!["personal", "replied"])
        .from("chris@thesogu.com")
        .with_output_type(Threads)
        .build();
    // println!("{:?}", command);

    let result = command.exec();

    if result.is_ok() {
        println!("{:?}", result);
    } else {
        println!("shit what the fuck happened")
    }
}

fn do_show() {
    let command = Show::new()
        .with_tags(vec!["personal"])
        .from("chris@thesogu.com")
        .and()
        .arg("not")
        .with_tag("replied")
        .build();
    let result = command.exec();
    if result.is_ok() {
        println!("{:?}", result);
    } else {
        println!("shit what the fuck happened");
        println!("{:?}", command);
        println!("{:?}", result);
    }
}

fn main() {
    // do_search();
    do_show();
}
