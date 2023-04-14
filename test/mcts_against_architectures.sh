#!/bin/zsh

typeset -a rewriter_only print_used_rules
zparseopts -D -E -rewriter-only=rewriter_only -print-used-rules=print_used_rules

if [[ $# -lt 3 ]]; then
	echo "Usage: $0 <Architecture parameters file> <Loops benchmark directory> <output directory> [optional extra flags to pass to run_tests_against.sh]"
	exit 1
fi

param_file=$1
bmarks=$2
output=$3
shift 3

# This script doesn't need this parameter as much because
# these only require 9,000 compiles per benchmark
# rather than the 9 million or so required by the full
# benchmarking run.
fraction_to_run=1.0

# The print-used-rules flag only works for the egraph rewriter backend iirc.
# Note that Egg (Egrpah library) is a bit buggy, so enabling this reduces
# gthe successful compile rate.
rewriter_extra_flags=""
if [[ ${#print_used_rules} -gt 0 ]]; then
	rewriter_extra_flags="$extra_flags --print-used-rules"
fi

mkdir -p $output/stdout
mkdir -p $output/temp_architecture_mcts


echo "Staring $param_file  with rewriter"
./mcts_tests_against.sh $fraction_to_run $param_file $bmarks $output/temp_architecture_rewriter --use-rewriter $@ &> $output/stdout/rewriter.out
# cp $output/temp_architecture_rewriter/run_output $output/rewriter.out
# rm -rf $output/temp_architecture_rewriter
