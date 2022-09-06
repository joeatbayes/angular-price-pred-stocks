


pub mod bar_parser {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;
    use crate::trade::trade as trade;
    //use num_traits::cast::ToPrimitive;
    //use std::collections::HashMap;

    // The output is wrapped in a Result to allow matching on errors
    // Returns an Iterator to the Reader of the lines of the file.
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

 
  
    #[derive(Debug)]
    pub struct Bars { 
        pub dayn : Vec<f32>,
        pub date : Vec<String>,
        pub open : Vec<f32>,
        pub high : Vec<f32>,
        pub low  : Vec<f32>,
        pub close : Vec<f32>,
        pub adj_close: Vec<f32>,
        pub vol  : Vec<i64>
    }

    impl Bars {
        pub fn new() -> Bars {
            Bars {
               dayn: Vec::with_capacity(5000),
               date: Vec::new(),
               open: Vec::new(),
               high: Vec::new(),
               low: Vec::new(),
               close: Vec::new(),
               adj_close: Vec::new(),
               vol: Vec::new()
            }
        }

        pub fn len(&self) -> usize {
            self.date.len()
        } 
 
        // ensure start and end indexes for a slice will not exceed
        // the slide or do silly things like ask for a end after the 
        // start.
        pub fn fix_slice_parm(&self, begn : usize, endn: usize) -> (usize, usize) {
            let slen = self.len();

            let endn =  if  endn >= slen { slen -1}
               else {endn};

            let startn = if begn <= 0 {0}
              else if begn > endn {endn}
              else {begn};

              return (startn, endn)
        }

        pub fn slice_dayn(&self, begn: usize, endn :usize) -> &[f32] {
            let (startn, endn) = self.fix_slice_parm(begn, endn);            
            return &self.dayn[startn..endn];
        }

        pub fn slice_date(&self, begn: usize, endn :usize) -> &[String] {
            let (startn, endn) = self.fix_slice_parm(begn, endn);            
            return &self.date[startn..endn];
        }

        pub fn slice_close(&self, begn: usize, endn :usize) ->  &[f32] {
            let (startn, endn) = self.fix_slice_parm(begn, endn);            
            return &self.close[startn..endn];
        }

        
        pub fn trade_stats_simple_hold(&self, hold_bars : i32, pstart_ndx : usize, pend_ndx : usize) -> trade::TradeStats {
            let mut win_cnt = 0;
            let mut loss_cnt= 0;
            let mut win_tot = 0.0;
            let mut loss_tot= 0.0;
            let start_ndx = pstart_ndx.max(0) as usize;
            let end_ndx = pend_ndx.min(self.len() -1) as usize;
            for ndx in start_ndx .. end_ndx {
                let sell_ndx = (ndx + hold_bars as usize).min(end_ndx);
                let buy_price = self.close[ndx];
                let sell_price= self.close[sell_ndx];
                let net = sell_price - buy_price;  
                if net > 0.0 {
                    //println!("profitable");
                    win_cnt += 1;
                    win_tot += net;
                } else {
                    //println!("not profitable");
                    loss_cnt += 1;
                    loss_tot += net;
                } // else
            } // for pair
            //println!("win_cnt={0:#?} win_tot={1:#?}", win_cnt, win_tot);
            return trade::make_trade_stats(win_cnt, win_tot, loss_cnt, loss_tot);
        } // fn
         


    } // impl
   



    pub fn read_file(fname: &str) -> Bars {
        // File data/test-file.txt must exist this produces output
        let mut dta = Bars::new(); 
        println!("fiName={0:#?}",fname);
        if let Ok(lines) = read_lines(&fname) {
            // Consumes the iterator, returns an (Optional) String   
            let mut lc : i32 = 0;
            for line in lines {
                lc = lc + 1;
                if lc <= 1 {
                   continue; 
                }
                
                if let Ok(uwline) = line {
                    let uwline = uwline.trim();
                    //println!("{}", uwline);
                    let sp: Vec<&str> = uwline.split(",").collect();
                    if sp.len() >= 7 {
                        //println!("date={0:#?} open={1:#?} ", &sp[0], &sp[1]); 
                        dta.dayn.push(lc as f32);
                        dta.date.push(sp[0].to_string());
                        dta.open.push(sp[1].parse().unwrap_or(-1.0));
                        dta.high.push(sp[2].parse().unwrap_or(-1.0)); 
                        dta.low.push(sp[3].parse().unwrap_or(-1.0));
                        dta.close.push(sp[4].parse().unwrap_or(-1.0));
                        dta.adj_close.push(sp[5].parse().unwrap_or(-1.0));
                        dta.vol.push(sp[6].parse().unwrap_or(-1));
                    } 
                }
            }
        }
        return dta
    }

}

