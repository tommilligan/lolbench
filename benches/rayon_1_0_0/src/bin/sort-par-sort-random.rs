//{"name":"sort::par_sort_random","crate":"rayon_1_0_0"}
extern crate rayon_1_0_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; rayon_1_0_0 :: sort :: par_sort_random ( & mut crit ) ; }