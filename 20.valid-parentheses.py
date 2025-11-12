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
    def isValid(self, s: str) -> bool:

        queue = Deque()
        dict = {}
        dict["("] = ")"
        dict["["] = "]"
        dict["{"] = "}"

        brackets = {"(", ")", "[", "]", "{", "}"}

        for char in s:
            # only act here if char is a bracket
            if char in brackets:

                if not queue:  # if the queue is empty, add char
                    queue.append(char)
                    continue

                top_of_queue = queue[-1]
                dict_finder = dict.get(top_of_queue)

                if dict_finder == char:
                    queue.pop()
                    continue
                else:
                    queue.append(char)

        if not queue:
            return True
        return False


# @leet end
