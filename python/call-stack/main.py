import inspect
import pprint


def printVariables(*args):
    for arg in args:
        print("%12s:%s" % (arg, inspect.stack()[1][0].f_locals[arg]))


def func_a():
    func_a_v_1: bool = True
    func_a_v_2: str = "hello A"

    func_b()


def func_b():
    func_b_v_1: bool = False
    func_b_v_2: str = "hello B"

    func_c()


def func_c():
    # print("%12s:%s" % (arg, inspect.stack()[1][0].f_locals[arg]))
    print(inspect.stack()[1][0].f_locals['func_b_v_1'])
    print(inspect.stack()[1][0].f_locals['func_b_v_2'])
    print(inspect.stack()[2][0].f_locals['func_a_v_1'])
    print(inspect.stack()[2][0].f_locals['func_a_v_2'])
    for idx, s in enumerate(inspect.stack()):
        print(f"idx: {idx}")
        pprint.pp(s.frame.f_locals)
        print()



if __name__ == "__main__":
    func_a()