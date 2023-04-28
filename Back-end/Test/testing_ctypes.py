import ctypes

test_so_file = "../Test/test.so" # Running from Spyrals-FastAPI
clibrary = ctypes.CDLL(test_so_file)

''' Test 1 '''
clibrary.test1()

''' Test 2 '''
clibrary.test2("test",2) # can only pass in a character with ctypes

''' Test 3 '''
clibrary.test2(b"test", 2) #Can only pass in binary strings
# In python strings are immutable - can't be modified. In c, strings are mutable.

''' Test 3 '''
# Alternate method for calling the function 
func = clibrary.test3
# c types - character pointer and int
func.restype = ctypes.c_char_p # There can only be 1 return type, so this isn't a list

test3 = func(b"test", 2).decode()
print(test3)

''' Test 4 '''
func = clibrary.test4
func.argtypes = [ctypes.c_char_p] 
func.restype = ctypes.c_char_p

test4 = func(b"hello", b"world").decode()
print(test4)

''' Test 5 '''
func = clibrary.test5
# c types - character pointer and int
func.argtypes = [ctypes.c_char_p, ctypes.c_int]
func.restype = ctypes.c_char_p

test5 = func(b"test", 5).decode()
print(test5)