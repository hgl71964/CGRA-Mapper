These are the commands that get issued to run the benchmarks.

For the case studies, see ../benchmark_scripts/

For the cross-comparisons and compile times, run:

./run_all_benchmarks_slurm.sh ../Loops4 500 0.2

(Note that 0.2 and 500 are sampling parameters to reduce the amount of computation required: 500 is how many architectures to test, and 0.2 is the fraction of loops to test against each architecture.)


Plotting:

python plot_successes.py successes.png run_all_benchmarks_outputs/no_rules_data run_all_benchmarks_outputs/llvm_data run_all_benchmarks_outputs/greedy_data run_all_benchmarks_outputs/rewriter_data


Test note:

./compile.sh kernel.cpp  # generate kernel.bc
./run.sh ../build/src/libMapperPass.so kernel.bc --build  # generate operations.json for the kernel.bc
./build_param.sh operations.json # build the settings file for this new loop.
./run.sh ../build/src/libMapperPass.so kernel.bc  --param-file param.json --use-egraphs  # perform rewrite

example email:


For example, lets say we have two different loops:

kernel1.c:

...
for (...) {
   x[i] = x[i] + 1;
}

kernel2.c:
for(...) {
  x[i] = x[i] - 1;
}

We can build an architecture for kernel1.c using ./run.sh ... kernel1.bc --build, which creates the operations.json that /that loop/ needs.  Then, (due to my clunky tooling) we create a param.json file using ./build_param.sh param_skeleton operations.json param.json
That script adds some other stuff.

Then, we can build kernel2.c, compiling for that architecture using ./run.sh ... kernel2.bc --param-file param.json --use-egraphs

and it will try and rewrite that x[i] -1 into x[i] + (-1) since the - operation won't be available on that CGRA.


