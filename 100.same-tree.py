# @leet imports start
from string import *
from re import *
from datetime import *
from collections import *
from heapq import *
from bisect import *
from copy import *
from math import *
from random import *
from statistics import *
from itertools import *
from functools import *
from operator import *
from io import *
from sys import *
from json import *
from builtins import *
import string
import re
import datetime
import collections
import heapq
import bisect
import copy
import math
import random
import statistics
import itertools
import functools
import operator
import io
import sys
import json
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
    def isSameTree(self, p: Optional[TreeNode], q: Optional[TreeNode]) -> bool:
        # code here

        def dfs(node, val_storage):
            if not node:
                val_storage.append("None")
                return

            val_storage.append(node.val)

            dfs(node.left, val_storage)
            dfs(node.right, val_storage)

        p_vals = []
        q_vals = []

        dfs(p, p_vals)
        dfs(q, q_vals)

        print(f"p vals: {p_vals}")
        print(f"q vals: {q_vals}")

        if p_vals == q_vals:
            return True

        return False

        
# @leet end
