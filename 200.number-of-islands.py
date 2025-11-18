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
    def numIslands(self, grid: List[List[str]]) -> int:
        found_land = set()
        island_count = 0

        def greedy_search(grid, row, col, found_land) -> bool:
            row_valid = 0 <= row < len(grid)
            col_valid = 0 <= col < len(grid[0])
            # is_one = grid[row][col] == "1"
            not_found = (row, col) not in found_land

            # print(f"row: {row}, col: {col}")
            # print(f"row_valid: {row_valid}, col_valid: {col_valid}, not_found: {not_found}")

            if row_valid and col_valid and not_found and grid[row][col] == "1":
                found_land.add((row, col))

                left = greedy_search(grid, row, col-1, found_land)
                right = greedy_search(grid, row, col+1, found_land)
                up = greedy_search(grid, row-1, col, found_land)
                down = greedy_search(grid, row+1, col, found_land)

                return True
            else: 
                return False

        for row in range(len(grid)):
            for col in range(len(grid[0])):
                # print(f"row: {row}, col: {col}")
                if greedy_search(grid, row, col, found_land):
                    island_count += 1

        print(found_land)
        return island_count

        
# @leet end
