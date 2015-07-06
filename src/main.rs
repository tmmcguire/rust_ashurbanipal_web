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
    let mut style_rec = style.scored_results(773).unwrap();
    style_rec.sort_by( |&(_,l),&(_,r)| l.partial_cmp(&r).unwrap() );

    let topic = topic::Topic::read(&args[2]);
    let mut topic_rec = topic.scored_results(773).unwrap();
    topic_rec.sort_by( |&(_,l),&(_,r)| l.partial_cmp(&r).unwrap() );

    for &(etext,dist) in topic_rec.iter() {
        println!("{} {}", etext, dist);
    }
}
