pub mod matcher { 
    // Attempt to find the set of angles that most 
    // closely match the existing angle so we can determine
    // if there is merrit in the idea that similar pairs 
    // of lines will yield similar future movement in prices
    // and if does this provide sufficient lift to allow it's
    // use as a trading indicator.   This module is roughly modeled 
    // on the concept of a KNN engine where get_similar() will
    // find the set of points most similar to the specified angle pair.
    // and build_similarity_matrix() will find the most similar 
    // points for a set of angle pair.

    extern crate string_builder;
    use string_builder::Builder;
    use std::string::FromUtf8Error;
    //use crate::reg_fit::reg_fit as rfit;
    use crate::reg_fit::reg_fit::BNDPair;
    use crate::trade::trade::TradeStats;

    // Store the set of pairs most similiar to the
    // master pair.  The matches are listed in most
    // similar firest. Where the amount of similarity
    // is judged by the sim_score methods of BNDPair
    // BestNumDayFit. 
    #[derive(Debug,Clone)]
    pub struct BNDMatch {
        pub master : BNDPair,
        pub matches : Vec<BNDPair>
    }


    // search an sorted array using binary search and return 
    // the index of the matching element or the point where 
    // can not be found. 
    // todo - convert this to generic function with any structure
    // and a compare function. 
    pub fn bfind(arr : &Vec<BNDPair>, cmp : &BNDPair) -> usize {
        let mut maxn : usize = arr.len();
        let mut minn : usize = 0;
        let mut last_ndx : usize = 0;
       
        loop {
            let ndx : usize = (maxn + minn) / 2;
            let ae = arr[ndx];
            //println!("bfind ndx={0:#?} maxn={1:#?} minn={2:#?} aesloper={3:#?} cmpsloper={4:#?} lastndx={5:#?}", 
            //    &ndx, &maxn, &minn, &ae.long_line.sloper, &cmp.long_line.sloper, &last_ndx);
            if maxn == minn {
                // can not search any lower
                return ndx; 
            } else if ndx == last_ndx {
              // hysteris where the bars are one line apart and 
              // can never break the stalemate with integer division
              return ndx; 
            } else if cmp.long_line.sloper < ae.long_line.sloper {
               // search value is less than test node so search left 1/2
               maxn = ndx;    
            } else if cmp.long_line.sloper > ae.long_line.sloper {
               // search value is higher than test node so search right 1/2 
               minn = ndx;        
            } else {
                // either found the search value 
                // TOOD: Add support for multiple matching left by scanning 
                // left until do not find a match.
                return ndx;
            };
            //println!("bfind ndx={0:#?} lastndx={1:#?}", &ndx, &last_ndx);
            last_ndx = ndx;

    
 
        } // loop
    } // fn

   

    pub fn get_similar(pairs : &Vec<BNDPair>, mpair : &BNDPair) -> Vec<BNDPair> {
        let look_out = 70;
        let max_overlap = 0.3;
        let num_to_keep = 10;
        //let end_ndx = ndx + look_out;
        let last_ndx = pairs.len() -1;
        let mut sims : Vec<BNDPair> = Vec::new();
        // find the item with the closest matching long slope
        //println!("start bfind mpair={0:#?}", &mpair);
        let ndx = bfind(&pairs, &mpair);
        //println!("bfind ndx={0:#?} mpair={1:#?}", ndx, &mpair);
        // capture close by items and score them.
        for icnt  in 0 .. look_out {
            let lndx = ndx - icnt.min(ndx);
            let hndx = (ndx + icnt).min(last_ndx);
            let mut lpair = pairs[lndx];
            let mut hpair = pairs[hndx];
            let loverlap = mpair.overlap(&lpair);
            let hoverlap = mpair.overlap(&hpair);
            //println!("loverlap={0:#?} hoverlap={1:#?}", &loverlap, &hoverlap);
            if loverlap < max_overlap {
               lpair.score = mpair.sim_score(lpair);
               sims.push(lpair.clone());
            }
            if hoverlap < max_overlap {
               hpair.score = mpair.sim_score(hpair);
               sims.push(hpair.clone());
            }
        } // for icnt

        // Sort the candidates based on their matching scores
        sims.sort_by_key(|x| ((0.0 - (x.score * 100000.0)) as i64));
        // and keep just the best matches 
        //println!("sims={0:#?}", &sims);

        //  Collect the best matching items that do not overlap
        // too much with either the main item or other higher
        // scored items. 
        let mut tout : Vec<BNDPair> = Vec::new();
        for sim in sims { 
           // while eliminating any lower score candidates that
           // overlap the main or any higher score candidates by 
           //  over 30%.
           let moverlap = mpair.overlap(&sim);
           if moverlap > max_overlap {
              continue;
           }
           // check to see if new candidate overlaps with other 
           // items we already decided to keep
           let mut dokeep = true;
           for keeper in &tout {
               let koverlap = sim.overlap(&keeper);
               if koverlap > max_overlap {
                  dokeep = false;
                  break;
               } 
            }
            if dokeep {
                tout.push(sim);
                if tout.len() > num_to_keep {
                    break;
                }
            }
        }
        //println!("keepers={0:#?}", &tout);
        return tout;
    }

    pub fn get_matches(pairs : &Vec<BNDPair>, mpair : &BNDPair)  -> BNDMatch {
      let sims = get_similar(pairs, mpair);
      return  BNDMatch {
          master : mpair.clone(),
          matches : sims
        };
    } // fn

    // capture the sims for every element
    // This could run into-excess memory usage 
    // if we try to store all of them due to the copy
    // behavior of rust.      
    pub fn build_similarity_matrix(pairs : &Vec<BNDPair>) -> Vec<BNDMatch> {
        let mut tout : Vec<BNDMatch> = Vec::new();
        for mpair in pairs {
            let matched = get_matches(pairs, &mpair);
            tout.push(matched);
        } // for mpair
        tout.sort_by_key(|x| (x.master.short_line.end_ndx));
        return tout;
    } // fn

  


    pub fn safe_div(num : f32, divisor : f32, default : f32) -> f32{
        if divisor == 0.0 {
            return default;
        }
        else {
            return num / divisor;
        }
    }

    pub fn match_stats(bmatch : &BNDMatch) -> TradeStats {
        let mut win_cnt = 0;
        let mut loss_cnt= 0;
        let mut win_tot = 0.0;
        let mut loss_tot= 0.0;
        for pair in &bmatch.matches {
            if pair.fp_dif_rat > 0.0 {
                win_cnt += 1;
                win_tot += pair.fp_dif_rat;
            } else {
                loss_cnt += 1;
                loss_tot += pair.fp_dif_rat;
            } // else
        } // for pair
        let tot_cnt  = loss_cnt + win_cnt;
        let win_avg  = safe_div(win_tot, win_cnt as f32, 0.0);
        let loss_avg = safe_div(loss_tot, loss_cnt as f32, 0.0);
        let win_rat  = safe_div(win_cnt as f32, tot_cnt as f32, 0.0);
        let loss_rat = safe_div(loss_cnt as f32, tot_cnt as f32, 0.0);
        let win_net  = win_tot + loss_tot;
        let avg_net  = safe_div(win_net, tot_cnt as f32, 0.0);
        let appt =   (win_rat * win_avg) - (loss_rat.abs() * loss_avg.abs());

        
        return TradeStats { 
            win_tot : win_tot,
            win_cnt : win_cnt,
            win_rat : win_rat,
            win_avg : win_avg,
            loss_tot : loss_tot,
            loss_cnt : loss_cnt,
            loss_rat : loss_rat.abs(),
            loss_avg : loss_avg.abs(),
            net_tot  : win_net,
            net_avg  : avg_net,
            appt     : appt
          }
    }


     // produce a human friendly columnuar report showing
     // contents of all the BND Pairs
     pub fn as_rep_string(dta : &Vec<BNDMatch>) -> Result<String, FromUtf8Error> {
        let mut b = Builder::default();
        let mut spc = 99;

        fn print_det(label : &String, pair : &BNDPair) -> String {
            let sl = pair.short_line;
            let ll = pair.long_line;
            return format!("{label:6} {lslope:7.3}% {llen:5} {loffset:9.3} {lend:7} {sslope:7.3}% {slen:5} {soffset:9.3} {send:7} {angle:11.2} {drat:7.2}% {score:9.4}\n",
                label = label, lslope=ll.sloper * 100.0,  llen=ll.num_ele, loffset=ll.offset, lend=ll.end_ndx,
                sslope=sl.sloper * 100.0, slen=sl.num_ele, soffset=sl.offset, send=sl.end_ndx,
                angle=pair.angle, drat=(pair.fp_dif_rat*100.0), score=pair.score);           
        } // fn

        for bmatch in dta {
            spc += 1;
            if spc > 3 {
                b.append("\n");
                b.append("           long  long      long    long    short short     short    short  intersect  fp dif      match\n");
                b.append("          slope   len    offset     end    slope   len    offset      end      angle    Perc      score\n");
                b.append("------ -------- ----- --------- ------- -------- ------ -------- -------- ---------- -------  ----------\n");
                spc = 0;
            }
            let s = print_det(&"master".to_string(), &bmatch.master);
            b.append(s);
            for pair in &bmatch.matches {
                let s = print_det(&" ..".to_string(), pair);
                b.append(s);
            } // for pair
           let ss = match_stats(bmatch);
           b.append(format!("         #win={0:2}  #loss= {1:3} winRat= {2:5.2}% lossRat={3:5.2}% avg_net= {4:5.4}%\n",
                ss.win_cnt, ss.loss_cnt, (ss.win_rat*100.0), (ss.loss_rat*100.0), (ss.net_avg*100.0)));
            b.append(format!("         tot_win= {0:7.2}% avg_win= {1:7.3}% net_P&L= {2:5.3}%\n",
                 ss.win_tot*100.0, ss.win_avg*100.0, ss.net_tot*100.0));
            b.append(format!("         tot_loss= {0:7.2}% avg_loss= {1:7.3}% appt= {2:5.3}% \n",
                  ss.loss_tot*100.0, ss.loss_avg*100.0, ss.appt*100.0 ));
            //b.append(format!(""))
            b.append("\n");
        } // for
        return b.string();
        
    } // fn


} // mod