#!/usr/bin/env python3

'''
A classifier for SysY test cases.
'''

import json


CLASSIFIER = [
    ('一元运算', lambda x: x['grammar']['unary_exprs']),
    ('乘除模运算', lambda x: x['grammar']['mul_exprs']),
    ('加减运算', lambda x: x['grammar']['add_exprs']),
    ('比较运算', lambda x: x['grammar']['rel_exprs'] or x['grammar']['eq_exprs']),
    ('逻辑运算', lambda x: x['grammar']['land_exprs']
     or x['grammar']['lor_exprs']),
    ('常量', lambda x: x['grammar']['const_decls']),
    ('变量', lambda x: x['grammar']['var_decls']),
    ('赋值', lambda x: x['grammar']['assign_stmts']),
    ('语句块', lambda x: x['grammar']['block_stmts']),
    ('分支', lambda x: x['grammar']['if_stmts']
     or x['grammar']['if_else_stmts']),
    ('循环', lambda x: x['grammar']['while_stmts']),
    ('函数', lambda x: x['grammar']['void_func_defs']
     or x['grammar']['int_func_defs'] > 1 or x['grammar']['func_calls']),
    ('全局量', lambda x: x['grammar']['const_decls'] +
     x['grammar']['var_decls'] > x['grammar']['local_decls']),
    ('参数', lambda x: x['grammar']['func_params_max']
     > 4 or x['grammar']['func_array_params']),
    ('数组', lambda x: x['grammar']['const_array_defs'] or x['grammar']
     ['var_array_defs'] or x['grammar']['func_array_params'] or
     x['grammar']['array_accesses']),
]


def print_table_header():
  '''
  Prints the table header.
  '''
  print(f'文件名,{",".join(map(lambda x: x[0], CLASSIFIER))}')


def print_row(entry):
  '''
  Prints a row in the table.
  '''
  t_or_f = map(lambda x: '✓' if x[1](entry['stat']) else ' ', CLASSIFIER)
  print(f'{entry["file"]},{",".join(t_or_f)}')


def print_table(data):
  '''
  Prints the table.
  '''
  print_table_header()
  for entry in data['statistics']:
    print_row(entry)


if __name__ == '__main__':
  import sys
  if len(sys.argv) != 2:
    data = json.load(sys.stdin)
  else:
    with open(sys.argv[1], 'r') as f:
      data = json.load(f)
  print_table(data)
