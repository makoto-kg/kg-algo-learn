import random
import time
import math

_times = 10000000

def pi():
    _time_start = time.time()

    _cnt = 0
    for i in range(_times):
        _x = random.random()
        _y = random.random()

        #if _x ** 2 + _y ** 2 <= 1
        if _x * _x + _y * _y <= 1:
            _cnt += 1
    
    print(4.0 * _cnt / _times)

    _time_end = time.time()

    _time_split = _time_end - _time_start
    print(_time_split)

    return _time_split


def main():
    _test = 100

    _ave = 0.0

    for _i in range(_test):
        print('{:03d} times'.format(_i + 1))
        _ave += pi()
    
    print('ave: {} s'.format(_ave/_test))


if __name__ == "__main__":
    main()
