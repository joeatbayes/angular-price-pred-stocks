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
    //use string_builder::Builder;
    //use std::string::FromUtf8Error;
    //use crate::reg_fit::reg_fit as rfit;
    use crate::reg_fit::reg_fit::BNDPair;

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
        let look_out = 200;
        let max_overlap = 0.3;
        let num_to_keep = 8;
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
        sims.sort_by_key(|x| ((0.0 - (x.score * 10000000.0)) as i64));
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


    // capture the sims for every element
    // This could run into-excess memory usage 
    // if we try to store all of them due to the copy
    // behavior of rust.      
    pub fn build_similarity_matrix(pairs : &Vec<BNDPair>) -> Vec<BNDMatch> {
        let mut tout : Vec<BNDMatch> = Vec::new();
        for mpair in pairs {
            let sims = get_similar(pairs, mpair);
            let matched : BNDMatch = BNDMatch {
                master : mpair.clone(),
                matches : sims
            };
            tout.push(matched);
        } // for mpair
        return tout;
    } // fn


     // produce a human friendly columnuar report showing
     // contents of all the BND Pairs
     /*
     pub fn as_rep_string(dta : &Vec<BNDMatch>) -> Result<String, FromUtf8Error> {
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
    */


} // mod