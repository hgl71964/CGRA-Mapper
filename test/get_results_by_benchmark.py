import argparse

# Add to a dict
def add(d, v):
    if v in d:
        d[v] += 1
    else:
        d[v] = 1

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("LoopName") # Just for insertion into the result csv
    parser.add_argument("InputFile")
    parser.add_argument("OutputFile")
    parser.add_argument('--accelerator-size', required=True, type=int, help='The size of the accelerator --- for drawing the by-size graph.', dest='accelerator_size')

    args = parser.parse_args()

    # This file contains a bunch of junk,
    # followed by a bunch of lines
    # like:
    # [Mapping:success]
    # File Done: <fname>
    # or
    # [Mapping:fail]
    # File Done: <fname>
    # We turn this into a dict of
    # <benchmark_name>: successes and fails
    with open(args.InputFile) as f:
        success = False
        fail = False

        successes_dict = {}
        fails_dict = {}

        # Skip the loop compiling to itself --- sometimes cgra-mapper fails to map
        # the original loop to a cgra at all, so just skip those.
        for line in f.readlines()[2:]:
            if "Mapping:success" in line:
                success = True
            if "Mapping:fail" in line:
                fail = True
            if "Done File" in line:
                fname = line.split(':')[1]
                if fname.strip() == 'kernel.cpp':
                    # fname is the kernle th eloop was
                    # originally designed for
                    # different index because this doesn't get
                    # kenerl_ prefixed onto it
                    bname = args.LoopName.split('_')[1]
                else:
                    # print(fname.split('_'))
                    bname = fname.split('_')[1] # at 2 because the script prefixes kernel_ onto things
                if success:
                    add(successes_dict, bname)
                    success = False
                if fail:
                    add(fails_dict, bname)
                    fail = False

    # Now, do the output of this
    with open(args.OutputFile, 'a') as f:
        # We do this not as a true CSV because it's possible
        # (although it shouldn't happen) that not all benchmarks
        # will be there for each run.
        string = args.LoopName + ":" + str(args.accelerator_size)

        for key in successes_dict:
            string += ",success:" + key + ":" + str(successes_dict[key])
        for key in fails_dict:
            string += ",fail:" + key + ":" + str(fails_dict[key])

        f.write(string + "\n")
