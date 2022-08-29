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
fn reg_line_fit_err(darr : &[f32], offset : usize, slope_rat :f32) -> f32 {
    let nele = darr.len();
    if nele < 2 {
        return -1.0;
    }
    let slope_unit = (offset as f32) * slope_rat;
    let mut cmp_val = darr[offset];
    let mut err_sum = 0.0;
    for bndx in 0 .. nele {
        let pval = darr[bndx];
        let pdif = pval - cmp_val;
        let dif_rat = pdif / cmp_val;
        let abs_dif_rat = dif_rat.abs();
        let psqr = abs_dif_rat * abs_dif_rat;
        err_sum += psqr;
        cmp_val = cmp_val + slope_unit;
    }
    return err_sum / (nele as f32);
}

//TODO:  Use enum parameter to select the data open,low,high,low,close
//TODO:  use a getSlice which implements the enum logic so we don't spread it around.
//
fn find_best_fit_in_range(pbars : bars::Bars, offset : usize, min_len : i32, max_len : i32) -> i32 {
    // Start at a current day then work backwards to find the trend length
    // between min,max days that yields the lowest error.
    for num_day in min_len .. max_len {
        let beg_day = (num_day as usize) - offset;
        let dayns = pbars.slice_dayn(beg_day, offset);
        let closes= pbars.slice_close(beg_day,offset);
        let val_at_offset = pbars.close[offset];
        println!("dayns={0:#?} closes={1:#?} ", dayns, closes); 
        let tpl1 : (f32, f32) = linear_regression(dayns, closes).unwrap_or((0.0,-0.001));
        println!("tpl1={0:#?}", tpl1);
  
        let (slope, offset) = tpl1;
        let slope_rat = slope / val_at_offset;
        println!("slope={0:#?}, offset={1:#?}, slope_ratio={2:#?}", slope, offset, slope_rat);
        let days_offset = 350;
        let fit_err = reg_line_fit_err(dayns, days_offset, slope_rat);
        println!("fit_err={0:#?}", fit_err);
    }
    return 1;
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

    let bfl = find_best_fit_in_range(pbars, 500, 10,  30); 
    println!("bfl={0:#?}", bfl);
    
    
    
   

}
