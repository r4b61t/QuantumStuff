import quantum_stuff as qs
import numpy as np

coeff = 1/np.sqrt(2)

state = qs.Register(2)
Hadamard = qs.QGate([coeff],[coeff],[coeff],[-coeff])
Xnot = qs.QGate([0],[1],[1],[0])

state.apply_gate(Hadamard,[0],[])
state.apply_gate(Hadamard,[1],[0])
print(state.measure())
