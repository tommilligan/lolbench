//{"name":"fibonacci::fibonacci_split_iterative","crate":"rayon_1_0_0"}
extern crate rayon_1_0_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; rayon_1_0_0 :: fibonacci :: fibonacci_split_iterative ( & mut crit ) ; }