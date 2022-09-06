pub mod bar_parser;
pub mod reg_fit;
pub mod angle_matcher;
pub mod back_test;
pub mod trade;

use crate::bar_parser::bar_parser as bars;
use crate::angle_matcher::matcher as matcher;
use crate::reg_fit::reg_fit as rfit;
use crate::back_test::back_test as btest;
use crate::trade::trade as trades;
use linreg::{linear_regression};
//use sysinfo::SystemExt::System;
//use sysinfo::Process;

// First we update all information of our system struct.


fn test_find_short_long_best_fit(pbars : &bars::Bars) {
    // find the short line
    let last_bar_ndx = 500;
    let bfl = rfit::find_best_fit_in_range(&pbars, last_bar_ndx, 7,  25); 
    println!("from best fit short function bfl={0:#?}", bfl);


    // find the longer line
    let long_start_ndx = (last_bar_ndx - bfl.num_ele as usize) as usize;
    let min_long_ele=  bfl.num_ele * 3;
    let max_long_ele = min_long_ele * 4;
    
    //min_long_ele = cmp::max(60,min_long_ele );
    //println!("min_long_ele={0:#?}  max_long_ele={1:#?}", min_long_ele,max_long_ele );
    let bf2 = rfit::find_best_fit_in_range(&pbars, long_start_ndx, min_long_ele,  max_long_ele); 
    println!("from best fit long  function bfl={0:#?}", bf2);

    let look_forward_bars = 14;
    let fut_price_ndx = last_bar_ndx + look_forward_bars;
    let curr_price = pbars.close[last_bar_ndx];
    let fut_price = pbars.close[fut_price_ndx];
    let fp_dif = fut_price - curr_price;
    let fp_dif_rat = fp_dif / curr_price;
    println!("fut_price={0:#?} curr_price={1:#?} dif={2:#?} dif_rat={3:#?}", 
       fut_price, curr_price, fp_dif, fp_dif_rat);
}


fn main() {
    //let mut system = System::new();
    //system.refresh_all();

    //let pbars = bars::read_file(&"../data/bars/SPY.csv");
    let pbars = bars::read_file(&"../data/bars/RSP.csv");
    //println!("pbars={0:#?}", pbars);

    // TODO: change this so slice asks for columns by enum
    //  which can return vec of int, float, string depending
    //  on what enum is asked for. // https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
    //let dayns = pbars.slice_dayn(1,350);
    //let closes= pbars.slice_close(1,350);
    //println!("dayns={0:#?} closes={1:#?} ", dayns, closes);
     
      
    //let tpl1 : (f32, f32) = linear_regression(dayns, closes).unwrap_or((0.0,-0.001));
    //println!("tpl1={0:#?}", tpl1);
    //let (slope, offset) = tpl1;
    //let slope_rat = slope / offset;
    //println!("slope={0:#?}, offset={1:#?}, slope_ratio={2:#?}", slope, offset, slope_rat);
    
   
    //let tot_num_bar = pbars.len();

    //   test_find_short_long_best_fit(&pbars);
    
    // TODO:  We want to be able to process for only a slice
    //  of pbars especially when testing Eg:  only process
    //  bars 500 to 700 out of a set that may be thousands 
    //  of bars long. 
    //let bpair =  rfit::best_fit_angle(&pbars, 500, 12, 60);
    //println!("bpair={0:#?}", bpair);
    let min_short =20;
    let max_short =30;
    let port_set_for_test = 0.55;
    let max_test_size = 365 as usize;
    let num_test_ele = ((pbars.len() as f32 * port_set_for_test) as usize).min(max_test_size);
    let num_tran_ele =  (pbars.len() - num_test_ele) -1; 
    let first_test_ndx = (pbars.len() - num_test_ele) -1;
    let last_test_ndx = (pbars.len() - 1) as usize;
    let first_train_ndx = (max_short * 2) as usize;
    let last_train_ndx = first_test_ndx -1;

    let angles_for_all = rfit::build_fit_angles(
          &pbars, first_train_ndx, last_train_ndx, 
          min_short, max_short);
    
    let num_test_bars = last_test_ndx - first_test_ndx;
    let num_train_bars = last_train_ndx - first_train_ndx;

    //print!("angles_for_all={0:#?}", angles_for_all);
    // find the set of bpair which are most similar by score
    // skipping those that overlap with an existing item by 
    // more than 20% of all underlying samples. 
    // fn find_most_similar(allbp : Vec<BNDPair> ) -> Vec<BNDPair> {
    //
    //} 


    // For now we will do this with a simple sort but eventually
    // we want to use the similarity of slope onn a long slope 
    // sorted array and do a binary search (bisect_left_by) to find 
    // the most similar slope. Then we can move out on both sides 
    // checking the similarity and overlap rules until we get the desired
    // number of matches.
    // match-k-nearest-neighbors 
    //for bpair in &angles_for_all {
    //    let most_sim = find_most_similar(bpair, &angles_for_all); 
    //}

    //let rep_str = rfit::as_rep_string(&angles_for_all).unwrap();
    //println!("{0}", rep_str);

    let matches = matcher::build_similarity_matrix(&angles_for_all);
    //println!("matches = {0:#?}", &matches);

    // report that shows the keepers for each bar
    // with basic stats.  What I want to see is if
    // the keepers have clustering that is materially 
    // different than the underlying base probability.
    // if the therory holds true we should see this clustering.
    let rep_str = matcher::as_rep_string(&matches).unwrap();
    println!("{0}", rep_str);

    // Basic combination for the similar keepers.
    // Need to know the base probability of bars rising and dropping.
    // combine the rising bars based on keepers and falling bars based
    // on keepers For each of these we can get a raw vote based 
    // basic percentage.  What we really want is the basic probability
    // less the base probability it is the differential as lift
    // that represents something better.

    // And finally the RAM and SWAP information:
    //println!("total memory: {} kB", system.get_total_memory());
    //println!("used memory : {} kB", system.get_used_memory());
    // TODO:  exended this to print data process runtime and memory
    // usage for this process from Struct sysinfo::Process
    // https://tikv.github.io/doc/sysinfo/index.html

    let trades = btest::back_test(&angles_for_all,&pbars,first_test_ndx as usize , 
        last_test_ndx as usize,  min_short, max_short);
    let tstats = btest::trade_stats(&trades);
    println!("btr={0:#?}", &trades);
    println!("trade stats={0:#?}", &tstats);
    println!("base stats simple hold N days={0:#?}",  pbars.trade_stats_simple_hold( 15, first_test_ndx as usize,last_test_ndx as usize));
    println!("number of test_bar={0:#?}  # train bar={1:#?}", num_test_bars, num_train_bars);


} // main
