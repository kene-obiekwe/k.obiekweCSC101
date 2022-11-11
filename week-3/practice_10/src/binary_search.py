def binary_search(a_list,target):
	first = 0
	last = len(a_list) -1
	found = -1
	while first <= last and found == -1:
		midpoint = (first + last)//2
		if a_list[midpoint] == target:
			found = TRUE
		else:
			if target < a_list[midpoint]:
				last = midpoint - 1
			else:
				first = midpoint + 1

	return found

list = [5,15,25,35,45,67,79,80]
print(binary_search(list,list.index(7)))

