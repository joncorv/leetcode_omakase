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
    def maxProfit(self, prices: List[int]) -> int:
        num_prices = len(prices)

        if num_prices <=1:
            return 0

        max_profit = 0
        # lowest price seen is the first price
        lowest_price = prices[0]

        # iterate starting on second price
        for price in prices[1:]:
            if price < lowest_price:
                lowest_price = price
                continue
            elif price - lowest_price > max_profit:
                max_profit = price - lowest_price

        return max_profit



        
# @leet end
