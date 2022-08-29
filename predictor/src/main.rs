pub mod bar_parser;
use crate::bar_parser::bar_parser as parser;
use linreg::{linear_regression};

fn main() {
    println!("Hello, world!");

    let hm = parser::read_file(&"../data/bars/SPY.csv");
    println!("hm={0:#?}", hm);

    // TODO: change this so slice asks for columns by enum
    //  which can return vec of int, float, string depending
    //  on what enum is asked for. // https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
    let dayns = hm.slice_dayn(1,350);
    let closes= hm.slice_close(1,350);
    println!("dayns={0:#?} closes={1:#?} ", dayns, closes);
     
    //let xs: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    //let ys: Vec<f64> = vec![2.0, 4.0, 5.0, 4.0, 5.0];
    //assert_eq!(Ok((0.6, 2.2)), linear_regression(&xs, &ys));
   
    let x: (f32, f32) = (12.0,18.0);
    //let x: (f32, f32)  = linear_regression(dayns, closes).unwrap(); 
    //let  y: Result((f32, f32)) = linear_regression(dayns, closes);  
    
    let tpl1 : (f32, f32) = linear_regression(dayns, closes).unwrap_or((0.0,-0.001));
    println!("tpl1={0:#?}", tpl1);

    let (slope, offset) = tpl1;
    let slope_rat = slope / offset;
    println!("slope={0:#?}, offset={1:#?}, slope_ratio={2:#?}", slope, offset, slope_rat);

        let tpl : (f32, f32) = match linear_regression(dayns, closes) {    
        Ok(tpl) => { println!("val={0:#?}", tpl);
                      tpl 
                   },
        _ => { println!("not ok"); 
             (0.0,-0.001)
            }
    };

    // Compute a fit fitness value that can be used
    // to compute how well the data points from a 
    // linear regression cluster around the line. It 
    // is specifically modified to work with ratio of
    // difference to allow comparison across different 
    // time frames.  It is also modified to penalize
    // large differences from the line even if there 
    // is a matching set on the other side of the line.
     
    fn calc_err(darr : &[f32], offset : f32, slope_rat :f32) -> f32 {
       let nele = darr.len();
       let slope = offset * slope_rat;
       let cmp_val = offset;
       let mut err_sum = 0.0;
       for bndx in 0 .. nele {
           let pval = darr[bndx];
           let pdif = pval - cmp_val;
           let dif_rat = pdif / cmp_val;
           let abs_dif_rat = dif_rat.abs();
           let psqr = abs_dif_rat * abs_dif_rat;
           err_sum += psqr;
           let cmp_val = cmp_val + slope;
       }
       return err_sum / (nele as f32);
    }

    /*
    // Start at a current day then work backwards to find the trend length
    // between min,max days that yields the lowest error.
    let short_min_days = 10
    let short_max_days = 30
    let day_ndx = 350 
    for num_day in short_min_days .. short_max_days {
        let beg_day = day_ndx - num_day
        let dayns = hm.slice_dayn(beg_day, day_ndx);
        let closes= hm.slice_close(beg_day,day_ndx);
        println!("dayns={0:#?} closes={1:#?} ", dayns, closes); 
        let tpl1 : (f32, f32) = linear_regression(dayns, closes).unwrap_or((0.0,-0.001));
        println!("tpl1={0:#?}", tpl1);
    
        let (slope, offset) = tpl1;
        let slope_rat = slope / offset;
        println!("slope={0:#?}, offset={1:#?}, slope_ratio={2:#?}", slope, offset, slope_rat);
        


        
    } 
    */
    
   

}
