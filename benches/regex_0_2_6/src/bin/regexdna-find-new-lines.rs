//{"name":"regexdna::find_new_lines","crate":"regex_0_2_6"}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: regexdna :: find_new_lines ( & mut crit ) ; }