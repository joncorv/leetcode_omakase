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
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        counter = dict()
        result = list()
        winners = list()

        for i in range(len(nums) + 1):
            result.append(None)

        for num in nums:
            current_count = counter.get(num)
            if current_count:
                counter[num] = current_count + 1
            else:
                counter[num] = 1

        for num, freq in counter.items():
            result_frequency = result[freq]
            if result_frequency:
                result_frequency.append(num)
            else:
                result[freq] = [num]


        while k>= 0:
            for x in result[::-1]:
                if x:
                    for z in x:
                        winners.append(z)
                        k -= 1

        return winners[:k]


# @leet end

