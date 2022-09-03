//pub mod super::bar_parser;


pub mod reg_fit {
    // Use linear regression to build find the number of days
    // which provide the best (lowest error) fit for a short 
    // line and a long liine preceding a specified bar.   
    // Also includes data structures and helper methods to 
    // repeat this process for a set subset of bars to allow 
    // bulk analysis.  The output of this pass is generally 
    // the primary input for angle_matcher::matcher

    use crate::bar_parser::bar_parser as bars;
    //use crate::bar_parser::bar_parser as bars;
    use linreg::{linear_regression};
    extern crate string_builder;
    use string_builder::Builder;
    use std::string::FromUtf8Error;

    // Stores a single line description developed from 
    // multiple linear regression passes over a base set
    // of floating point data normally a column of stock
    // prices
    #[derive(Debug,Copy,Clone)]
    pub struct BestNumDayFit {
        pub sloper : f32,
        pub num_ele: i32,
        pub err : f32,
        pub offset : f32,
        pub end_ndx : i32,
        pub slope : f32
    }

    // Stores a pair of lines the long trend line
    // and short trend line plus the angle of intersection 
    // between the two lines and a single forward looking
    // projection showing how the prices moved next 
    #[derive(Debug,Copy,Clone)]
    pub struct BNDPair {
        pub long_line : BestNumDayFit,
        pub short_line : BestNumDayFit,
        pub angle : f32,
        pub fp_dif_rat : f32,
        pub score: f32 // used after matching process 
    }



    

    impl BestNumDayFit {
      // compute a similarity score by compareing  two lines 
      // based on how close the slope which is reduced by
      // how differnt the lengths of the lines are. 
      pub fn sim_score(&self, cmp : BestNumDayFit) -> f32 {
        let ssim = 20.0 - ((10.0 + self.sloper) - (10.0 + cmp.sloper));
        //let num_ele_dif = (self.num_ele - cmp.num_ele).abs();
        //let num_ele_rat = num_ele_dif as f32 / (self.num_ele + cmp.num_ele) as f32;
        //let num_ele_score = 1.0 - (num_ele_rat * 0.3);
        //return ssim * num_ele_score // reduce angle score so larger # of ele difference reduces score even more  }
         
        return ssim;
      }
    }

    impl BNDPair {
        // compute a similar score between two sets of 
        // long / short lines based on slope and and 
        // line lenghts.  The long line is 3 times more
        // important in this score than the short line.
        pub fn sim_score(&self, cmp : BNDPair) -> f32 {
          let lscore = self.long_line.sim_score(cmp.long_line);
          let sscore = self.short_line.sim_score(cmp.short_line);
          return (lscore + (sscore * 0.9)) / 2.0;
          //return lscore;
          //let adif = (180.0 + self.angle) - (180.0 + cmp.angle);
          //let asum = self.angle.abs() + cmp.angle.abs();
          //let arat = (adif / asum)*2.0;
          //return lscore * (1.0 - (arat * 2.8)); 
        } 

        // compute a similarity score between two BNDpair based
        // and the difference in angle and length of total line
        // analyzed short & long. The therory is that what we
        // really care about is the angle at intersection of 
        // the lines rather than their absolue slopes.  Not sure
        // if it makes total sense. but it does reduce the scoring
        // to only two weighted factors.   It could have an issue
        // a slight bend down from downward trend being viewed the
        // same as a slight bend less positive on a upward tragetory.
        pub fn sim_score_angle(&self, cmp : BNDPair) -> f32 {
            let dur1 = self.long_line.num_ele + self.short_line.num_ele;
            let dur2 = cmp.long_line.num_ele + cmp.short_line.num_ele;
            let ddif = dur1 - dur2;
            let dsum = dur1 + dur2; // use dsum for ratio to accomodate gaurantee scale from 0 to 1.
            let drat = 1.0 - (((ddif as f32) / (dsum as f32))*2.0); 
                // when comparing against dsum we have effectively doubled our line len
                // use the 1 - to force a small difference to act as a small discount
                // against the angle score. 
            let adif = self.angle - cmp.angle;
            let asum = self.angle + cmp.angle;
            let arat = (adif / asum)*2.0;
            let mut ascore = arat * drat * 1000.0;

            // If slope of ain point is upward then change magnitude
            // of score to force into separate realm from the negative
            // slopes. 
            if self.long_line.slope > 0.00001 {
               ascore = ascore * 1000.0;
            }
            return ascore;
        } 

        // figure out how much of the lines for two BNDPair overlap
        // with each other and return that number.
        pub fn overlap(&self, cmp : &BNDPair) -> f32 {
           let begn = self.long_line.end_ndx - self.long_line.num_ele;
           let endn = self.short_line.end_ndx;

           let begc = cmp.long_line.end_ndx - cmp.long_line.num_ele;
           let endc = cmp.short_line.end_ndx;

           if endn < begc {
                return 0.0
           }
           else if begn > endc {
                return 0.0;
           } else {
                // figure out how much is poking out the front
                let fndur : i32 = if begn < begc && endn > begc {
                    begc - begn
                } else {
                    0
                };

                // figure out how much is poking out the front
                let endur : i32 = if endn > endc && begn < endc {
                    endn - endc
                } else {
                    0
                };
                let num_overlap = self.long_line.num_ele - (fndur + endur);
                //println!("overlap begn={0:#?} endn={1:#?} begc={2:#?} endc={3:#?} fndur={4:#?} endur={5:#?} num_overlap={6:#?} numele={7:#?}",
                //    &begn, &endn, &begc, &endc, &fndur, &endur, &num_overlap, self.long_line.num_ele);
                return (num_overlap as f32) / (self.long_line.num_ele as f32);
           }
        }
    }


     // produce a human friendly columnuar report showing
     // contents of all the BND Pairs
     pub fn as_rep_string(dta : &Vec<BNDPair>) -> Result<String, FromUtf8Error> {
            let mut b = Builder::default();
            let mut spc = 99;
            for bpair in dta {
                spc += 1;
                if spc > 50 {
                    b.append("\n");
                    b.append("   long  long      long    long     short short     short    short intersect   fp dif\n");
                    b.append("  slope   len    offset     end     slope   len    offset      end     angle     Perc\n");
                    b.append("------- ----- --------- ------- --------- ------ -------- -------- ---------- -------\n");
                    spc = 0;
                }
                let sl = bpair.short_line;
                let ll = bpair.long_line;
                let lstr = format!("{lslope:8.5} {llen:5} {loffset:9.3} {lend:7} {sslope:9.5} {slen:5} {soffset:9.3} {send:7} {angle:9.2} {drat:7.2}%\n",
                    lslope=ll.sloper,  llen=ll.num_ele, loffset=ll.offset, lend=ll.end_ndx,
                    sslope=sl.sloper, slen=sl.num_ele, soffset=sl.offset, send=sl.end_ndx,
                    angle=bpair.angle, drat=(bpair.fp_dif_rat*100.0));
                b.append(lstr)
            } // for
            return b.string();
            
     } // fn
    

    // Compute a line fit fitness value that can be used
    // to compute how well the data points from a 
    // linear regression cluster around the line. It 
    // is specifically modified to work with ratio of
    // difference to allow comparison across different 
    // time frames.  It is also modified to penalize
    // large differences from the line even if there 
    // is a matching set on the other side of the line.
    pub fn reg_line_fit_err(darr : &[f32], offset : f32, slope_rat :f32) -> f32 {
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


    //TODO:  Use enum parameter to select the data open,low,high,low,close
    //TODO:  use a getSlice which implements the enum logic so we don't spread it around.
    // Find the length of line within the specified range for the given end_ndx
    // which yields the lowest error average between the regression line and 
    // the data points. 
    pub fn find_best_fit_in_range(pbars : &bars::Bars, end_ndx : usize, min_len : i32, max_lenp : i32) -> BestNumDayFit {
        // Start at a current day then work backwards to find the trend length
        // between min,max days that yields the lowest error.
        let num_ele = pbars.len();
        let max_len :i32 = max_lenp.min((num_ele as i32)-1).min(end_ndx as i32);
        //println!("max_lenp={0:#?} max_len={1:#?}", max_len, max_lenp);
        let mut best : BestNumDayFit = BestNumDayFit {
            sloper: 0.0, 
            num_ele : -1, 
            err : 99999999.99, 
            offset : 0.0,
            end_ndx: (end_ndx as i32),
            slope : 0.0
        };
    
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
            //let days_offset = 350;

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
    } //fn

    // Compute the degree angle between two sloped lines. 
    pub fn calc_angle_from_slope(slope1 : f32, slope2 : f32) -> f32 {
        // use the arc tangent angle formula  then multiply by 57.3 to get degrees
        return ((slope1 - slope2) /(1.0 + (slope1 * slope2))).atan() * 57.3;
    }

    
    //  compute the long and short line best fit line find finding
    // the longest line with the lowest error average from the 
    // regression line the data points. Calculate the angle of 
    // those lines and return both the BNDpair structure.
    pub fn best_fit_angle(pbars : &bars::Bars, last_bar_ndx : usize, min_short : i32, max_short : i32) -> BNDPair {
        // find the longer line
        let bfl = find_best_fit_in_range(&pbars, last_bar_ndx, min_short,  max_short); 
        //println!("from best fit short function bfl={0:#?}", bfl);

        let long_start_ndx = (last_bar_ndx - bfl.num_ele as usize) as usize;
        let min_long_ele=  ((bfl.num_ele as f32) * 3.0) as i32;
        let max_long_ele = ((min_long_ele as f32) * 1.2) as i32;
        
        //min_long_ele = cmp::max(60,min_long_ele );
        //println!("min_long_ele={0:#?}  max_long_ele={1:#?}", min_long_ele,max_long_ele );
        let bf2 = find_best_fit_in_range(&pbars, long_start_ndx, min_long_ele,  max_long_ele); 
        //println!("from best fit long  function bfl={0:#?}", bf2);

        let look_forward_bars = 3;
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
        let tout : BNDPair = BNDPair {
            long_line : bf2,
            short_line : bfl,
            fp_dif_rat : fp_dif_rat,
            angle : angle,
            score : 0.0
        };
        return tout;
    }

    // for every available bar compute the long trend lines with the 
    // shorter change line creating pair of lines and return a vector
    // of those those for all data points except those too early in 
    // the data set to have valid trend lines.
    pub fn build_fit_angles(pbars : &bars::Bars, min_short :  i32, max_short : i32 ) -> Vec<BNDPair> {
       
       let mut tout : Vec<BNDPair> = Vec::new();
       let first_ndx = max_short * 2;
       let last_ndx = (pbars.len() as i32) - 1;
       for last_bar_ndx in  first_ndx .. last_ndx {
           //println!("last_bar_ndx={0:#?}, ", last_bar_ndx);
           let bfa = best_fit_angle(&pbars, last_bar_ndx as usize, min_short, max_short);
           //println!("last_bar_ndx={0:#?}, bfa={1:#?}", last_bar_ndx, bfa);
           tout.push(bfa);
       }   
       //tout.sort_by_key(|x| ((x.angle * 1000000.0) as i64));
       tout.sort_by_key(|x| ((x.long_line.sloper * 1000000.0) as i64));
       return tout;
    }

    //enum CmpRes {
    //    Gt,
    //    Le,
    //    Eq
    //}

   
} // mod