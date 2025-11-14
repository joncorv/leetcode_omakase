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
    def maxArea(self, height: List[int]) -> int:

        largest_area = 0
        left, right = 0, len(height) - 1

        while left < right:
            # if moving the left pointer in is bigger area than before
            current_width = right - left
            current_height = min(height[left], height[right])
            current_area = current_width * current_height

            largest_area = max(current_area, largest_area)

            if height[left] < height[right]:
                left += 1
            else:
                right -= 1

        return largest_area


# @leet end
