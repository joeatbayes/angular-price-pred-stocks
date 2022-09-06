pub mod trade {
    use crate::angle_matcher::matcher::safe_div;
    
    #[derive(Debug, Copy, Clone)]
    pub struct TradeStats {
        pub win_tot : f32,
        pub win_cnt : i32,
        pub win_rat : f32,
        pub win_avg : f32,
        pub loss_cnt : i32,
        pub loss_tot : f32,
        pub loss_rat : f32,
        pub loss_avg : f32,
        pub net_tot : f32,
        pub net_avg : f32,
        pub appt : f32  // https://www.investopedia.com/articles/forex/07/profit_loss.asp
    }

    pub fn make_trade_stats(win_cnt : i32, win_tot : f32, loss_cnt : i32, loss_tot : f32) -> TradeStats {
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
    } // fn
}  // mod