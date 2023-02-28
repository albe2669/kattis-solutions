import sys

# Merge sort algorithm based on Robert Sedgewick's implementation
def merge(arr, aux, lo, mid, hi):
    swaps = 0
    i,j = lo, mid+1

    aux[lo:hi+1] = arr[lo:hi+1]

    for k in range(lo, hi+1):
        if i > mid:
            arr[k] = aux[j]
            j += 1
        elif j > hi:
            arr[k] = aux[i]
            i += 1
        elif aux[j] < aux[i]:
            arr[k] = aux[j]
            j += 1
            swaps += (mid - i + 1)
        else:
            arr[k] = aux[i]
            i += 1

    return swaps

def sort(arr, aux, lo, hi):
    if hi <= lo:
        return 0

    mid = lo + (hi - lo)//2

    return sort(arr, aux, lo, mid) + sort(arr, aux, mid+1, hi) + merge(arr, aux, lo, mid, hi)

def mergesort(arr):
    aux = arr.copy()
    return sort(arr, aux, 0, len(arr)-1)
    
students = [int(x) for x in sys.stdin.read().split()][1:]
print(mergesort(students))

