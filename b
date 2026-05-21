s0	|	Vtype=Shift(2)	END=Reduce(2)	||	VDecl=3	Code=1
s1	|	END=Accept	||
s2	|	Id=Shift(5)	||	Assign=6
s3	|	END=Reduce(2)	Vtype=Shift(2)	||	VDecl=3	Code=4
s4	|	END=Reduce(0)	||
s5	|	Id=Shift(10)	Semi=Shift(9)	||	Assign=8
s6	|	END=Reduce(1)	Semi=Shift(7)	||
s7	|	END=Reduce(4)	Vtype=Reduce(4)	||
s8	|	END=Reduce(5)	Semi=Reduce(5)	||
s9	|	END=Reduce(3)	Vtype=Reduce(3)	||
s10	|	Id=Shift(10)	||	Assign=8

TokenManager {
  tokens:
    [7] Assign (non-terminal)
    [8] Id (terminal)
    [6] Vtype (terminal)
    [9] Semi (terminal)
    [0] START (terminal)
    [3] UNDEFINED (terminal)
    [1] EPSILON (terminal)
    [4] Code (non-terminal)
    [5] VDecl (non-terminal)
    [2] END (terminal)
  productions:
    (0) Code -> VDecl Code
    (1) Code -> Vtype Assign
    (2) Code ->
    (3) VDecl -> Vtype Id Semi
    (4) VDecl -> Vtype Assign Semi
    (5) Assign -> Id Assign
}
