pub mod settings  {
    pub struct Config {
        pub hold_bars : i32,
        pub knn_max_overlap : f32,
        pub knn_look_out : usize,
        pub knn_num_to_keep : usize
    }


    impl Config {
        pub fn default() -> Config {
            return Config {
               hold_bars : 15,

               knn_max_overlap : 0.3,
               knn_look_out : 70,
               knn_num_to_keep: 10
            }
        } // fn
    } // impl


} // mod