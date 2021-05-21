def sort(x: list, up):
    if len(x) <= 1:
        return x

    mid_point = len(x) // 2
    first = sort(x[:mid_point], True)
    second = sort(x[mid_point:], False)

    x1 = first + second

    return _sub_sort(x1, up)


def _sub_sort(x, up):
    if len(x) == 1:
        return x

    _compare_and_swap(x, up)

    mid_point = len(x) // 2
    first = _sub_sort(x[:mid_point], up)
    second = _sub_sort(x[mid_point:], up)

    return first + second


def _compare_and_swap(x, up):
    mid_point = len(x) // 2

    for i in range(mid_point):
        if (x[i] > x[mid_point + i]) == up:
            x[i], x[mid_point+i] = x[mid_point+i], x[i]


def main():
    nums = [10, 30, 11, 20, 4, 330, 21, 110]

    result = sort(nums, True)

    print(nums)
    print(result)


if __name__ == '__main__':
    main()
