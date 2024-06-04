export function applyIfFloat<T1, T2>(
    val: T2,
    func: (n: number) => T1
): T1 | T2 {
    if (typeof val == "number" && val % 1 != 0) return func(val);

    return val;
}
