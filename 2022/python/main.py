import argparse
import day6.solution

parser = argparse.ArgumentParser()
parser.add_argument("-d", "--day", type=int)

args = parser.parse_args()

match args.day:
    case 6:
        day6.solution.part1()
        day6.solution.part2()

    case _:
        print("I haven't solved this day in Python yet :<")
