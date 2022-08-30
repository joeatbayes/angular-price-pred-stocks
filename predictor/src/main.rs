pub mod bar_parser;
use crate::bar_parser::bar_parser as bars;
use linreg::{linear_regression};

// Compute a line fit fitness value that can be used
// to compute how well the data points from a 
// linear regression cluster around the line. It 
// is specifically modified to work with ratio of
// difference to allow comparison across different 
// time frames.  It is also modified to penalize
// large differences from the line even if there 
// is a matching set on the other side of the line.
fn reg_line_fit_err(darr : &[f32], offset : f32, slope_rat :f32) -> f32 {
    let nele = darr.len();
    if nele < 2 {
        return -1.0;
    }
    let slope_unit = (offset as f32) * slope_rat;
    let mut cmp_val = offset as f32;
    let mut err_sum = 0.0;
    for bndx in 0 .. nele {
        let pval = darr[bndx];
        let pdif = pval - cmp_val;
        let dif_rat = pdif / cmp_val;
        let abs_dif_rat = dif_rat.abs();
        err_sum += abs_dif_rat;
        //println!("ndx={0:#?} pval={1:#?} cmp_val={2:#?} dif={3:#?} dif_rat={4:#?}", bndx,pval,cmp_val,pdif, dif_rat);
        cmp_val = cmp_val + slope_unit;
    }
    return err_sum / (nele as f32); 
}

#[derive(Debug,Copy,Clone)]
pub struct BestNumDayFit {
    pub sloper : f32,
    pub num_ele: i32,
    pub err : f32,
    pub offset : f32,
    pub end_ndx : i32,
    pub slope : f32
}

#[derive(Debug,Copy,Clone)]
pub struct BNDPair {
   pub long_line : BestNumDayFit,
   pub short_line : BestNumDayFit,
   pub angle : f32,
   pub fp_dif_rat : f32
}

impl BestNumDayFit {
  pub fn sim_score(&self, cmp : BestNumDayFit) -> f32 {
    let ssim = 1000.0 - (self.sloper - cmp.sloper);
    let num_ele_dif = (self.num_ele - cmp.num_ele).abs();
    let num_ele_rat = num_ele_dif as f32 / self.num_ele as f32;
    let num_ele_score = 1.0 - num_ele_rat;
    return ssim * num_ele_score // reduce angle score so larger # of ele difference reduces score even more  }
  }
}


//TODO:  Use enum parameter to select the data open,low,high,low,close
//TODO:  use a getSlice which implements the enum logic so we don't spread it around.
//
fn find_best_fit_in_range(pbars : &bars::Bars, end_ndx : usize, min_len : i32, max_lenp : i32) -> BestNumDayFit {
    // Start at a current day then work backwards to find the trend length
    // between min,max days that yields the lowest error.
    let num_ele = pbars.len();
    let max_len :i32 = max_lenp.min((num_ele as i32)-1).min(end_ndx as i32);
    //println!("max_lenp={0:#?} max_len={1:#?}", max_len, max_lenp);
    let mut best : BestNumDayFit = BestNumDayFit {sloper: 0.0, 
           num_ele : -1, 
           err : 99999999.99, 
           offset : 0.0,
           end_ndx: (end_ndx as i32),
           slope : 0.0};
   
    for num_day in min_len .. max_len {
        //println!("num_ele={4:#?} numDay={0:#?} min_len={1:#?} max_len={2:#?} end_ndx={3:#?}", num_day, min_len, max_len,  end_ndx, num_ele);
        let beg_ndx = end_ndx - (num_day as usize);
        let dayns = pbars.slice_dayn(0, num_day as usize);
        let closes= pbars.slice_close(beg_ndx, end_ndx);
        let beg_ndx_val = pbars.close[beg_ndx];
        //println!("dayns={0:#?} closes={1:#?} beg_ndx_val={2:#?} ", dayns, closes, beg_ndx_val); 
        let tpl1 : (f32, f32) = linear_regression(dayns, closes).unwrap_or((0.0,-0.001));
        //println!("numDay={0:#?},tpl1={01:#?}", num_day, tpl1);
        let (slope, offset) = tpl1;
        let slope_rat = slope / beg_ndx_val;
        //println!("slope={0:#?}, offset={1:#?}, slope_ratio={2:#?}", slope, offset, slope_rat);
        let days_offset = 350;

        let fit_err = reg_line_fit_err(closes, beg_ndx_val, slope_rat);
        //println!("fit_err={0:#?} best.err={1:#?} nday={2:#?}", fit_err, best.err, num_day);
        if fit_err < best.err {
            best.err = fit_err;
            best.num_ele = num_day;
            best.sloper = slope_rat;
            best.offset = offset;
            best.slope = slope;
            //println!("SET best_fit{0:#?}", best)
            //println!("new best num_day={0:#?} err={1:#?} slope={2:#?} offset={3:#?}",
            //    num_day, best.err, best.sloper, best.offset)
        }
    }
    return best;
}

  

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
    
   
    let tot_num_bar = pbars.len();

    fn test_find_short_long_best_fit(pbars : &bars::Bars) {
        // find the short line
        let last_bar_ndx = 500;
        let bfl = find_best_fit_in_range(&pbars, last_bar_ndx, 7,  25); 
        println!("from best fit short function bfl={0:#?}", bfl);
    

        // find the longer line
        let long_start_ndx = (last_bar_ndx - bfl.num_ele as usize) as usize;
        let mut min_long_ele=  bfl.num_ele * 3;
        //if min_long_ele < 60 { min_long_ele = 60}
        let mut max_long_ele = min_long_ele * 4;
        
        //min_long_ele = cmp::max(60,min_long_ele );
        //println!("min_long_ele={0:#?}  max_long_ele={1:#?}", min_long_ele,max_long_ele );
        let bf2 = find_best_fit_in_range(&pbars, long_start_ndx, min_long_ele,  max_long_ele); 
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

    fn calc_angle_from_slope(slope1 : f32, slope2 : f32) -> f32 {
        // use the arc tangent angle formula  then multiply by 57.3 to get degrees
        return ((slope1 - slope2) /(1.0 + (slope1 * slope2))).atan() * 57.3;
    }

    fn best_fit_angle(pbars : &bars::Bars, last_bar_ndx : usize, min_short : i32, max_short : i32) -> BNDPair {
        // find the longer line
        let bfl = find_best_fit_in_range(&pbars, last_bar_ndx, min_short,  max_short); 
        //println!("from best fit short function bfl={0:#?}", bfl);

        let long_start_ndx = (last_bar_ndx - bfl.num_ele as usize) as usize;
        let mut min_long_ele=  bfl.num_ele * 3;
        //if min_long_ele < 60 { min_long_ele = 60}
        let mut max_long_ele = min_long_ele * 4;
        
        //min_long_ele = cmp::max(60,min_long_ele );
        //println!("min_long_ele={0:#?}  max_long_ele={1:#?}", min_long_ele,max_long_ele );
        let bf2 = find_best_fit_in_range(&pbars, long_start_ndx, min_long_ele,  max_long_ele); 
        //println!("from best fit long  function bfl={0:#?}", bf2);

        let look_forward_bars = 14;
        let fut_price_ndx = (last_bar_ndx + look_forward_bars).min((pbars.len() ) -1);
        let curr_price = pbars.close[last_bar_ndx];
        let fut_price = pbars.close[fut_price_ndx];
        let fp_dif = fut_price - curr_price;
        let fp_dif_rat = fp_dif / curr_price;
        //println!("fut_price={0:#?} curr_price={1:#?} dif={2:#?} dif_rat={3:#?}", 
        //  fut_price, curr_price, fp_dif, fp_dif_rat);
        // use the arc tangent angle formula  then multiply by 57.3 to get degrees
        // subtract from 180 to flip angle we are looking at from lower left to 
        // upper where 180 dgree line is straight flat and 90 degree line is up.
        let angle = calc_angle_from_slope(bfl.slope, bf2.slope);
        return BNDPair {
            long_line : bf2,
            short_line : bfl,
            fp_dif_rat : fp_dif_rat,
            angle : angle
         }

    }

    fn build_fit_angles(pbars : &bars::Bars, min_short :  i32, max_short : i32 ) -> Vec<BNDPair> {
       let mut tout : Vec<BNDPair> = Vec::new();
       let first_ndx = max_short * 2;
       let last_ndx = (pbars.len() as i32) - 1;
       for last_bar_ndx in  first_ndx .. last_ndx {
           //println!("last_bar_ndx={0:#?}, ", last_bar_ndx);
           let bfa = best_fit_angle(&pbars, last_bar_ndx as usize, min_short, max_short);
           //println!("last_bar_ndx={0:#?}, bfa={1:#?}", last_bar_ndx, bfa);
           tout.push(bfa);
       }   
       tout.sort_by_key(|x| ((x.long_line.sloper * 1000000.0) as i64));
       return tout;
    }


    test_find_short_long_best_fit(&pbars);
    let bpair =  best_fit_angle(&pbars, 500, 12, 60);
    println!("bpair={0:#?}", bpair);

    let angles_for_all = build_fit_angles(&pbars, 12,60);
    print!("angles_for_all={0:#?}", angles_for_all);
}
