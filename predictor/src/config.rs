pub mod config {
   // constants and structures to control operation of the overall
   // angle fit and linear regression system in an attempt to 
   // isolate all important magic constants in a single location
   
   pub struct cfg {
      pub rfit_look_forward_bars : usize,
      pub rfit_short_min_len : usize,
      pub rfit_short_max_len : usize,
      pub rfit_long_min_mult : f32,
      pub rfit_long_max_mult : usize,

      pub knn_harvest_num : usize,
      pub knn_num_keep : usize,

      pub bars_beg_ndx : usize,
      pub bars_end_ndx : usize
   }

   pub fn default() -> cfg {

   }
   // todo load from disk

   // todo save to disk


}