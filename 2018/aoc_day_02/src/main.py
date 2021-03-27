
import time
import sys


def main(inputfile):
    with open(inputfile) as f:
        input_str = f.read()

        start = time.perf_counter() * 1000
        print(f"2a: {part_2a(input_str)}")
        after_a = time.perf_counter() * 1000
        print(f"2b: {part_2b(input_str)}")
        after_b = time.perf_counter() * 1000

        print(f"{after_a - start}, {after_b - after_a}")


def part_2a(input_str):
    for ln_a in input_str.splitlines():
        for ln_b in input_str.splitlines()[1:]:
            if len(ln_a) != len(ln_b):
                continue
            common_c = ""
            for c in filter(lambda t: t[0] == t[1], zip(ln_a, ln_b)):
                common_c += c[0]
            if len(common_c) == len(ln_a) - 1:
                return common_c

    return None
    

def part_2b(input_str):
    for ln_a in input_str.splitlines():
        for ln_b in input_str.splitlines()[1:]:
            if len(ln_a) != len(ln_b):
                continue
            common_c = ""
            diff_count = 0
            for a,b in zip(ln_a, ln_b):
                if a != b :
                    diff_count += 1
                    if diff_count > 1 :
                        break
                else:
                    common_c += a
            
            if diff_count == 1:
                return common_c

    return None
                



main(sys.argv[1])