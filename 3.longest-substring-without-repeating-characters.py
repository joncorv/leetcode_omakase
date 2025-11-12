# @leet imports start
import bisect
import collections
import copy
import datetime
import functools
import heapq
import io
import itertools
import json
import math
import operator
import random
import re
import statistics
import string
import sys
from bisect import *
from builtins import *
from collections import *
from copy import *
from datetime import *
from functools import *
from heapq import *
from io import *
from itertools import *
from json import *
from math import *
from operator import *
from random import *
from re import *
from statistics import *
from string import *
from sys import *
from typing import *

# @leet imports end

# @leet start
class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        num_chars = len(s)
        max_num_chars = 0

        if num_chars == 0:
            return 0
        elif num_chars == 1:
            return 1


        left, right = 0, 1

        while right < num_chars:
            test_string = s[left:right]
            search_char = s[right]
            test_string_finder = test_string.find(search_char) != -1
            print(f"checking if {test_string} has {search_char}. result: {test_string_finder}")
            
            if test_string_finder : #it finds the char at right
                if right - left -1 <=0: # if pointers are next to eachother
                    left += 1 
                    right += 1 
                else: 
                    left +=1
            else: 
                right +=1

            if right - left > max_num_chars:
                max_num_chars = right - left

        return max_num_chars
        
# @leet end
