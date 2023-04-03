import quantum_stuff as qs
import numpy as np

coeff = 1/np.sqrt(2)

state = qs.Register(3)
Hadamard = qs.QGate([coeff],[coeff],[coeff],[-coeff])
state.apply_gate(Hadamard,[0,1,2],[])
print(state.probabilities())
