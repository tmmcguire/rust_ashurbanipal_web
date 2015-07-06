mod mbitset;
mod recommendation;
mod style;
mod topic;

use std::env;

use recommendation::Recommendation;

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 3 { panic!("Usage: ashurbanipal_web pos-data topic-data"); }
    
    let style = style::Style::read(&args[1]);
    let style_rec = style.sorted_results(773).unwrap();

    let topic = topic::Topic::read(&args[2]);
    let topic_rec = topic.sorted_results(773).unwrap();

    for &(etext,dist) in topic_rec.iter() {
        println!("{} {}", etext, dist);
    }
}
