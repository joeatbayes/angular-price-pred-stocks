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
    pub offset : f32
}

//TODO:  Use enum parameter to select the data open,low,high,low,close
//TODO:  use a getSlice which implements the enum logic so we don't spread it around.
//
fn find_best_fit_in_range(pbars : bars::Bars, end_ndx : usize, min_len : i32, max_len : i32) -> BestNumDayFit {
    // Start at a current day then work backwards to find the trend length
    // between min,max days that yields the lowest error.
    let mut best : BestNumDayFit = BestNumDayFit {sloper: 0.0, 
           num_ele : -1, 
           err : 99999999.99, 
           offset : 0.0 };
   
    for num_day in min_len .. max_len {
        //println!("numDay={0:#?} min_len={1:#?} max_len={2:#?}", num_day, min_len, max_len);
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
            println!("SET best_fit{0:#?}", best)
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

    let bfl = find_best_fit_in_range(pbars, 500, 7,  30); 
    println!("from best fit function bfl={0:#?}", bfl);
    
    
    
   

}
