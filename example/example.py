import quantum_stuff as qs
import numpy as np

coeff = 1/np.sqrt(2)

state = qs.Register(8)
Hadamard = qs.QGate([coeff],[coeff],[coeff],[-coeff])

state.apply_gate(Hadamard,[0],[])
for i in range(7):
    state.apply_gate(Hadamard,[i+1],[i])

print(state.probabilities())
