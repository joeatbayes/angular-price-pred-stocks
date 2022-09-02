pub mod bar_parser;
pub mod reg_fit;
use crate::bar_parser::bar_parser as bars;
use crate::reg_fit::reg_fit as rfit;
use linreg::{linear_regression};


fn main() {
    println!("Hello, world!");

    let pbars = bars::read_file(&"../data/bars/SPY.csv");
    println!("pbars={0:#?}", pbars);

    // TODO: change this so slice asks for columns by enum
    //  which can return vec of int, float, string depending
    //  on what enum is asked for. // https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
    let dayns = pbars.slice_dayn(1,350);
    let closes= pbars.slice_close(1,350);
    println!("dayns={0:#?} closes={1:#?} ", dayns, closes);
     
      
    let tpl1 : (f32, f32) = linear_regression(dayns, closes).unwrap_or((0.0,-0.001));
    println!("tpl1={0:#?}", tpl1);
    let (slope, offset) = tpl1;
    let slope_rat = slope / offset;
    println!("slope={0:#?}, offset={1:#?}, slope_ratio={2:#?}", slope, offset, slope_rat);
    
   
    //let tot_num_bar = pbars.len();

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

       test_find_short_long_best_fit(&pbars);
    
    // TODO:  We want to be able to process for only a slice
    //  of pbars especially when testing Eg:  only process
    //  bars 500 to 700 out of a set that may be thousands 
    //  of bars long. 
    let bpair =  rfit::best_fit_angle(&pbars, 500, 12, 60);
    println!("bpair={0:#?}", bpair);

    let angles_for_all = rfit::build_fit_angles(&pbars, 9,45);
    print!("angles_for_all={0:#?}", angles_for_all);


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

    let rep_str = rfit::as_rep_string(&angles_for_all).unwrap();
    println!("{0}", rep_str);

    let matches = rfit::build_similarity_matrix(&angles_for_all);
    println!("matches = {0:#?}", &matches);


} // main
