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
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def levelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:

        if not root :
            return []
        
        even_level_nodes, odd_level_nodes = [], []
        result = []

        if root:
            even_level_nodes.append(root)

        while even_level_nodes or odd_level_nodes:
            current_level_vals = []

            if even_level_nodes:
                for node in even_level_nodes:
                    current_level_vals.append(node.val)
                    if node.left:
                        odd_level_nodes.append(node.left)
                    if node.right:
                        odd_level_nodes.append(node.right)
                result.append(current_level_vals)
                even_level_nodes.clear()
                continue
            else:
                for node in odd_level_nodes:
                    current_level_vals.append(node.val)
                    if node.left:
                        even_level_nodes.append(node.left)
                    if node.right:
                        even_level_nodes.append(node.right)
                result.append(current_level_vals)
                odd_level_nodes.clear()
                continue



        return result

# @leet end
