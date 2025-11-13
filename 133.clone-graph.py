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
"""
# Definition for a Node.
class Node:
    def __init__(self, val = 0, neighbors = None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []
"""

from typing import Optional


class Solution:
    def cloneGraph(self, node: Optional["Node"]) -> Optional["Node"]:

        seen = set()
        node_data = {}
        to_process = []

        if node:
            seen.add(node.val)
            to_process.append(node)

        while to_process:
            cur_node = to_process.pop()
            this_nodes_children = []

            if cur_node.neighbors:
                for child in cur_node.neighbors:
                    this_nodes_children.append(child.val)

                    if child.val not in seen:
                        to_process.append(child)
                        seen.add(child.val)

            node_data[cur_node.val] = this_nodes_children

        result = []
        for i in range(len(seen) -1):
            result.append( Node(i))

        for cur_node in result:
            neighbors = node_data.get(cur_node.val)

            if neighbors:
                for neigh in neighbors:
                    cur_node.neighbors.append(neigh)




        print(node_data)
        return result


# @leet end
