import quantum_stuff as qs
import numpy as np

coeff = 1/np.sqrt(2)

state = qs.Register(6)
Hadamard = qs.QGate([coeff],[coeff],[coeff],[-coeff])
Xnot = qs.QGate([0],[1],[1],[0])

state.apply_gate(Hadamard,[0],[])
for i in range(4):
    state.apply_gate(Hadamard,[i+1],[i])
state.apply_gate(Xnot,[0],[])
state.apply_gate(Hadamard,[5],[0,4])
print(state.probabilities())
