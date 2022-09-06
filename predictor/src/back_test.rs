pub mod back_test {
    //pub mod angle_matcher;
    //pub mod bar_parser;
    //pub mod reg_fit;
    use crate::reg_fit::reg_fit::BNDPair;
    //use crate::angle_matcher::matcher::BNDMatch;
    use crate::bar_parser::bar_parser as bars;
    use crate::angle_matcher::matcher as matcher;
    use crate::reg_fit::reg_fit as rfit;
    use crate::angle_matcher::matcher::safe_div;
    use crate::trade::trade::TradeStats;

    #[derive(Debug, Copy, Clone)]
    pub struct Trade {
        pub buy_ndx : i32,
        pub sell_ndx : i32,
        pub qty : i32,
        pub price : f32,
        pub cost : f32,
        pub sell_price : f32,
        pub proceeds : f32,
        pub net : f32,
        pub net_rat : f32,
        pub end_cap : f32
    }

    pub fn trade_stats(trades : &Vec<Trade>) -> TradeStats {
        let mut win_cnt = 0;
        let mut loss_cnt= 0;
        let mut win_tot = 0.0;
        let mut loss_tot= 0.0;
        for trade in trades {
            //println!("l32: net={0:#?} trade={1:#?}", trade.net, &trade);
            if trade.net > 0.0 {
                //println!("profitable");
                win_cnt += 1;
                win_tot += trade.net;
            } else {
                //println!("not profitable");
                loss_cnt += 1;
                loss_tot += trade.net;
            } // else
        } // for pair
        //println!("win_cnt={0:#?} win_tot={1:#?}", win_cnt, win_tot);
        let tot_cnt  = loss_cnt + win_cnt;
        let win_avg  = safe_div(win_tot, win_cnt as f32, 0.0);
        let loss_avg = safe_div(loss_tot, loss_cnt as f32, 0.0);
        let win_rat  = safe_div(win_cnt as f32, tot_cnt as f32, 0.0);
        let loss_rat = safe_div(loss_cnt as f32, tot_cnt as f32, 0.0);
        let win_net  = win_tot + loss_tot;
        let avg_net  = safe_div(win_net, tot_cnt as f32, 0.0);
        let appt =   (win_rat * win_avg) - (loss_rat.abs() * loss_avg.abs());
        let res = TradeStats { 
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
          };
        //println!("tradeRes={0:#?}", &res);
        return res;

    }



    //-- TODO: Make the trade decision a a separate part of the back test
    //     most likely by passing in a function pointer to the decision 
    //     maker so we can re-use backtest with many derivatives to 
    //     core logic.
    pub fn back_test(pairs : &Vec<BNDPair>,pbars : &bars::Bars, first_ndx : usize, last_ndx : usize, min_short : i32, max_short : i32) -> Vec<Trade> {
        let start_cap = 10000.0;     
        //let mut buy_ndx = 0;
        //let mut buy_price = 0;
        let hold_bars = 15;
        // This backtest is based on a simple premis of choose when to buy and always sell
        // at close hold_bars latter.   A better strategy may be to hold until the buy 
        // signal goes away or for hold_bars whichever is latter.
        let min_appt_for_buy =  0.002;  //0.003; 
        let min_win_rat_for_buy = 0.0; //0.6; 
        let min_net_avg_for_buy = 0.002;   //0.002; 
        let mut cap = start_cap.clone();
        let mut last_sell_ndx = 0;

        let max_ndx = last_ndx.min(pbars.len() - hold_bars);
        // for simple backtest skip any buying activity 
        // if there are not enough bars left to sell at close
        // hold_bars in the future. 
        let mut trades : Vec<Trade> = Vec::new();
        for ndx in  first_ndx .. max_ndx {
            //println!("last_bar_ndx={0:#?}, ", last_bar_ndx);            
            let bpair = rfit::best_fit_angle(&pbars, ndx as usize, min_short, max_short);
            let matches = matcher::get_matches(pairs, &bpair); // find the set of angles that best match the current pair
            let ms = matcher::match_stats(&matches);
            // don't have a trade open so check to see if we wanto to buy something.
            if (ms.appt > min_appt_for_buy && ms.win_rat > min_win_rat_for_buy 
                  && ms.net_avg > min_net_avg_for_buy 
                  && ndx > last_sell_ndx) {
                let sell_ndx = ndx + hold_bars;
                let buy_price = pbars.close[ndx];
                let sell_price = pbars.close[sell_ndx];
                //let qty = (cap / buy_price as f32) as i32;
                let qty = 10;
                let cost = buy_price * qty as f32;
                let proceeds = sell_price * qty as f32;
                let net = proceeds - cost;
                let net_rat = net / cost;
                cap = cap + net;
                last_sell_ndx = sell_ndx;
                let t = Trade  {
                  buy_ndx : ndx as i32,
                  sell_ndx: sell_ndx as i32,
                  qty : qty,
                  price : buy_price,
                  cost : cost,
                  sell_price : sell_price,
                  proceeds : proceeds,
                  net : net,
                  net_rat : net_rat,
                  end_cap : cap + 0.0
                };
                println!("L121: trade={0:#?}", &t);
                trades.push(t);
            } // if
        } // for ndx
        return trades;

    } // fn
    
} // mod