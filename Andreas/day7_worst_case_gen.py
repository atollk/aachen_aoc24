def check(n):
  s = str(n)
  t1 = int(s[-1:])
  t2 = int(s[-2:])
  if t1 != 0 and t1 != 1 and n % t1 == 0:
    return n//t1, int(s[:-1]), t1
  if t2 != 0 and t2 != 1 and n % t2 == 0:
    return n//t2, int(s[:-2]), t2
  return n, n, 0

def check_rec(n):
  if n < 100:
    return True
  a, b, c = check(n)
  print(n, a, b, c)
  if c == 0:
    return False
  return check_rec(a) and check_rec(b)

def collect_checks(n):
  if n < 100:
    return [n]
  a, b, c = check(n)
  if c == 0:
    raise ValueError()
  result = collect_checks(a)
  result.append(c)
  return result

def main():
  print(check_rec(29520))
  return

  import random

  good_cases = [x for x in range(10_000, 1_000_000) if check_rec(x)]
  good_cases = [x for x in good_cases]

  for x in good_cases:
    print(f"{x}: {' '.join(str(y) for y in collect_checks(x))}")

main()